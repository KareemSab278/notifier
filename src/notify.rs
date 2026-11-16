use expo_push_notification_client::{Expo, ExpoClientOptions, ExpoPushMessage, ExpoPushTicket};
/*
    make use of all cores in the cpu and spawn multiple threads for sending notifications concurrently
    speed is priority here

    run like:
    cargo run -- '[{
        "title": "Second test notification",
        "body": "This is the second test notification.",
        "push_token": "ExponentPushToken[ABCDEFG123456]"
    }]'
*/

#[derive(serde::Deserialize)]
struct Notification {
    title: String,
    body: String,
    push_token: String,
}

pub async fn send_notifications(recipients_json: &str) -> Result<(), Box<dyn std::error::Error>> {
    let notifications: Vec<Notification> = serde_json::from_str(recipients_json)?;
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
