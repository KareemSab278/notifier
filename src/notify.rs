use crate::structs::{Notification, NotificationRequestResult};
use expo_push_notification_client::{Expo, ExpoClientOptions, ExpoPushMessage, ExpoPushTicket};

const BATCH_SIZE: usize = 100;

pub async fn send_notifications(
    notifications: Vec<Notification>,
) -> Result<NotificationRequestResult, Box<dyn std::error::Error>> {
    let expo = Expo::new(ExpoClientOptions { access_token: None });

    let mut successful = 0usize;
    let mut failures = 0usize;

    for batch in notifications.chunks(BATCH_SIZE) {
        let mut messages = Vec::with_capacity(batch.len());

        for notification in batch {
            match ExpoPushMessage::builder([notification.push_token.as_str()])
                .title(&notification.title)
                .body(&notification.body)
                .build()
            {
                Ok(message) => messages.push(message),
                Err(_) => {
                    failures += 1;
                }
            };
        }

        if messages.is_empty() {
            continue;
        }

        let message_count = messages.len();
        match expo.send_push_notifications(messages).await {
            Ok(tickets) => {
                for ticket in tickets.iter() {
                    match ticket {
                        ExpoPushTicket::Ok(_) => successful += 1,
                        ExpoPushTicket::Error(_) => failures += 1,
                    }
                }

                failures += message_count.saturating_sub(tickets.len());
            }
            Err(_) => {
                failures += message_count;
            }
        }
    }

    Ok(NotificationRequestResult {
        success: successful,
        failed: failures,
    })
}

pub fn validate_notifications_payload(notifications: &[Notification]) -> Result<(), String> {
    for notification in notifications {
        if notification.body.is_empty() {
            return Err("Missing body".to_string());
        }
        if notification.title.is_empty() {
            return Err("Missing title".to_string());
        }
        if notification.push_token.is_empty() {
            return Err("Missing push token".to_string());
        }
    }
    Ok(())
}
