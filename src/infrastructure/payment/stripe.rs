use std::{collections::HashMap, env, str::FromStr};

use axum::http::HeaderMap;
use log::{error, warn};
use stripe::{
    BillingPortalSession, CheckoutSession, CheckoutSessionBillingAddressCollection,
    CheckoutSessionMode, CreateBillingPortalSession, CreateCheckoutSessionAutomaticTax,
    CreateCheckoutSessionCustomerUpdate, CreateCheckoutSessionCustomerUpdateAddress,
    CreateCustomer, CustomerId,
};
use stripe_webhooks::{StripeEvent, StripeListener};

use crate::{AppInfo, DbPool, domain::User};

#[derive(Clone)]
pub struct Stripe {
    db: DbPool,
    client: stripe::Client,
    success_url: String,
    cancel_url: String,
    website_url: String,
}
impl Stripe {
    pub fn new(app_info: &AppInfo, db: DbPool) -> Self {
        let secret_key = env::var("STRIPE_SECRET").expect("STRIPE_SECRET must be set");
        let client = stripe::Client::new(secret_key);

        Self {
            client,
            db,
            success_url: app_info.website_url.clone(),
            cancel_url: app_info.website_url.clone(),
            website_url: app_info.website_url.clone(),
        }
    }

    pub fn process_webhook_request(headers: &HeaderMap, body: &str) -> Result<StripeEvent, String> {
        let stripe = StripeListener::new(
            env::var("STRIPE_WEBHOOK_SECRET")
                .expect("Missing STRIPE_WEBHOOK_SECRET environment variable"),
        );
        stripe
            .process(headers, body)
            .inspect_err(|e| warn!("Error processing Stripe Webhook: {e}"))
            .map_err(|e| format!("Error processing event: {e}"))
    }

    pub async fn checkout(
        &self,
        user: &User,
        price_id: &str,
    ) -> Result<stripe::CheckoutSession, String> {
        let customer_id = self.create_customer(user).await?;

        let success_url = format!("{}/payments/successful", self.success_url);
        let cancel_url = format!("{}/payments/cancelled", self.cancel_url);

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
        let return_url = format!("{}/dashboard", self.website_url);

        let mut params = CreateBillingPortalSession::new(customer_id);
        params.return_url = Some(&return_url);

        BillingPortalSession::create(&self.client, params)
            .await
            .inspect_err(|e| error!("Unable to manage Stripe Subscription({}): {e}", user.email))
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
        .inspect_err(|e| error!("Unable to create Stripe Customer: {e}"))
        .map_err(|e| e.to_string())?;

        let customer_id = customer.id.to_string();

        sqlx::query(r#"UPDATE users SET stripe_customer_id = ? WHERE id = ?"#)
            .bind(customer_id)
            .bind(user.id)
            .execute(self.db.as_ref())
            .await
            .inspect_err(|e| error!("Unable to set Stripe Customer ID({}): {e}", user.email))
            .map_err(|e| e.to_string())?;

        Ok(customer.id)
    }
}
