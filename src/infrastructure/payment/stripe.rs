use std::{collections::HashMap, env, str::FromStr};

use sqlx::SqlitePool;
use stripe::{
    BillingPortalSession, CheckoutSession, CheckoutSessionBillingAddressCollection,
    CheckoutSessionMode, CreateBillingPortalSession, CreateCheckoutSessionAutomaticTax,
    CreateCheckoutSessionCustomerUpdate, CreateCheckoutSessionCustomerUpdateAddress,
    CreateCustomer, CustomerId,
};

use crate::{AppInfo, domain::User};

#[derive(Clone)]
pub struct Stripe {
    db: SqlitePool,
    app_info: AppInfo,
    client: stripe::Client,
}
impl Stripe {
    pub fn new(app_info: AppInfo, db: &SqlitePool) -> Self {
        let secret_key = env::var("STRIPE_SECRET").expect("STRIPE_SECRET must be set");
        let client = stripe::Client::new(secret_key);

        Self {
            client,
            app_info,
            db: db.clone(),
        }
    }

    pub async fn checkout(
        &self,
        user: &User,
        price_id: &str,
    ) -> Result<stripe::CheckoutSession, String> {
        let customer_id = self.create_customer(user).await?;

        let success_url = format!("{}/payments/successful", self.app_info.website_url);
        let cancel_url = format!("{}/payments/cancelled", self.app_info.website_url);

        let checkout_session = {
            let mut params = stripe::CreateCheckoutSession::new();
            params.cancel_url = Some(&cancel_url);
            params.success_url = Some(&success_url);
            params.customer = Some(customer_id);
            params.mode = Some(CheckoutSessionMode::Subscription);
            params.line_items = Some(vec![stripe::CreateCheckoutSessionLineItems {
                quantity: Some(1),
                price: Some(price_id.to_string()),
                ..Default::default()
            }]);
            params.automatic_tax = Some(CreateCheckoutSessionAutomaticTax {
                enabled: true,
                liability: None,
            });
            params.billing_address_collection =
                Some(CheckoutSessionBillingAddressCollection::Required);
            params.customer_update = Some(CreateCheckoutSessionCustomerUpdate {
                address: Some(CreateCheckoutSessionCustomerUpdateAddress::Auto),
                ..Default::default()
            });
            params.expand = &["line_items", "line_items.data.price.product"];

            CheckoutSession::create(&self.client, params).await.unwrap()
        };

        Ok(checkout_session)
    }

    pub async fn manage_subscription(&self, user: &User) -> Result<BillingPortalSession, String> {
        let customer_id = self.create_customer(user).await?;
        let return_url = format!("{}/dashboard", self.app_info.website_url);

        let mut params = CreateBillingPortalSession::new(customer_id);
        params.return_url = Some(&return_url);

        BillingPortalSession::create(&self.client, params)
            .await
            .map_err(|e| e.to_string())
    }

    async fn create_customer(&self, user: &User) -> Result<CustomerId, String> {
        if let Some(ref customer_id) = user.stripe_customer_id {
            return Ok(CustomerId::from_str(customer_id).unwrap());
        }

        let mut metadata = HashMap::new();
        metadata.insert("user_id".to_string(), user.id.to_string());
        metadata.insert("email".to_string(), user.email.clone());
        metadata.insert("full_name".to_string(), user.full_name.clone());

        let customer = stripe::Customer::create(
            &self.client,
            CreateCustomer {
                name: Some(&user.full_name),
                email: Some(&user.email),
                metadata: Some(metadata),
                ..Default::default()
            },
        )
        .await
        .map_err(|e| e.to_string())?;

        let customer_id = customer.id.to_string();

        sqlx::query(r#"UPDATE users SET stripe_customer_id = ? WHERE id = ?"#)
            .bind(customer_id)
            .bind(user.id)
            .execute(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(customer.id)
    }
}
