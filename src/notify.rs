use expo_push_notification_client::{Expo, ExpoClientOptions, ExpoPushMessage, ExpoPushTicket};

#[derive(serde::Deserialize)]
struct Notification {
    title: String,
    body: String,
    push_token: String,
}

pub async fn send_test_notification() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string("mock_data.json")?;
    let notifications: Vec<Notification> = serde_json::from_str(&data)?;
    let expo = Expo::new(ExpoClientOptions { access_token: None });
    let mut failures: i32 = 0;

    
    for notification in &notifications {
        let message = ExpoPushMessage::builder([notification.push_token.as_str()])
            .title(&notification.title)
            .body(&notification.body)
            .build()?;

        let tickets = expo.send_push_notifications(message).await?;

        for ticket in &tickets {
            match &ticket {
                ExpoPushTicket::Ok(_) => {}
                ExpoPushTicket::Error(error) => {
                    println!("Error sending test notification: {}", &error.message);
                    failures += 1;
                }
            }
        }
    }

    println!(
        "Finished sending {}/{} notifications - {} failures",
        (&notifications.len() - failures as usize),
        &notifications.len(),
        &failures
    );

    Ok(())
}
