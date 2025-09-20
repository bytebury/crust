use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    crust::start().await;
}
