use axum::{
    Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
};
use std::{env, sync::Arc};
use stripe_webhooks::{StripeEvent, StripeListener};

use crate::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/webhooks/stripe", post(stripe_webhook_listener))
}

pub async fn stripe_webhook_listener(
    State(_): State<Arc<AppState>>,
    headers: HeaderMap,
    body: String,
) -> impl IntoResponse {
    let stripe = StripeListener::new(
        env::var("STRIPE_WEBHOOK_SECRET")
            .expect("Missing STRIPE_WEBHOOK_SECRET environment variable"),
    );

    let events = match stripe.process(&headers, &body) {
        Ok(events) => events,
        Err(e) => {
            eprintln!("Error processing Stripe Event: {:?}", e);
            return (
                StatusCode::BAD_REQUEST,
                format!("Error processing event: {e}"),
            )
                .into_response();
        }
    };

    match events {
        StripeEvent::CheckoutSessionCompleted(value) => println!("{:?}", value),
        StripeEvent::CustomerSubscriptionDeleted(value) => println!("{:?}", value),
        StripeEvent::Unknown(value) => println!("{:?}", value),
    };

    StatusCode::OK.into_response()
}
