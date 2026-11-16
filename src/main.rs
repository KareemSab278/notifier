mod notify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    notify::send_test_notification().await
}
