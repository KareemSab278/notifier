mod notify;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let recipients = std::env::args()
        .nth(1)
        .ok_or("Usage: expo-notifications-r '<recipients-json>'")?;

    notify::send_notifications(&recipients).await
}
