use crate::structs::{Notification, NotificationRequestResult};
use expo_push_notification_client::{Expo, ExpoClientOptions, ExpoPushMessage, ExpoPushTicket};

const BATCH_SIZE: usize = 100;

pub async fn send_notifications(
    notifications: Vec<Notification>,
) -> Result<NotificationRequestResult, Box<dyn std::error::Error>> {
    let expo = Expo::new(ExpoClientOptions { access_token: None });

    let mut failed = Vec::new();

    for (batch_number, batch) in notifications.chunks(BATCH_SIZE).enumerate() {
        let mut messages = Vec::with_capacity(batch.len());
        let mut indexes = Vec::with_capacity(batch.len());

        for (offset, notification) in batch.iter().enumerate() {
            let index = u32::try_from(batch_number * BATCH_SIZE + offset)?;

            match ExpoPushMessage::builder([notification.push_token.as_str()])
                .title(&notification.title)
                .body(&notification.body)
                .build()
            {
                Ok(message) => {
                    messages.push(message);
                    indexes.push(index);
                }
                Err(_) => failed.push(index),
            };
        }

        if messages.is_empty() {
            continue;
        }

        match expo.send_push_notifications(messages).await {
            Ok(tickets) => {
                for (index, ticket) in indexes.iter().zip(tickets.iter()) {
                    match ticket {
                        // i can reduce processing by ignoring successful tickets and only tracking errors then comparing them to get the num of successes
                        ExpoPushTicket::Ok(_) => (),
                        ExpoPushTicket::Error(_) => failed.push(*index),
                    }
                }

                failed.extend(indexes.iter().skip(tickets.len()).copied());
            }
            Err(_) => {
                failed.extend(indexes);
            }
        }
    }

    failed.sort_unstable();

    Ok(NotificationRequestResult {
        success: notifications.len() - failed.len(),
        failed: failed,
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
