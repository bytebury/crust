use axum::{
    Router,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::post,
};
use std::sync::Arc;
use stripe_webhooks::StripeEvent;

use crate::{AppState, infrastructure::payment::stripe::Stripe};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new().route("/webhooks/stripe", post(stripe_webhook_listener))
}

pub async fn stripe_webhook_listener(
    State(_): State<Arc<AppState>>,
    headers: HeaderMap,
    body: String,
) -> impl IntoResponse {
    match Stripe::process_webhook_request(&headers, &body) {
        Ok(event) => {
            match event {
                StripeEvent::CheckoutSessionCompleted(value) => println!("{:?}", value),
                StripeEvent::CustomerSubscriptionDeleted(value) => println!("{:?}", value),
                StripeEvent::Unknown(value) => println!("{:?}", value),
            };
        }
        Err(e) => {
            eprintln!("Error processing Stripe Event: {:?}", e);
            return (
                StatusCode::BAD_REQUEST,
                format!("Error processing event: {e}"),
            )
                .into_response();
        }
    };

    StatusCode::OK.into_response()
}
