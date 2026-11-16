/*
    run like:
    cargo run -- '[{
        "title": "Second test notification",
        "body": "This is the second test notification.",
        "push_token": "ExponentPushToken[ABCDEFG123456]"
    }]'


    maybe i can make this listen all the time for any incoming requests like an api?
        then it sends out the notifications as they come in? 

    this would avoid startup loading and just work on the fly... hmm... 
    so just send the json to the api endpoint this service provides and thats it - should be handled automatically and instant.
    instead of invoking the rust program with php, php can just send the notification data to the API endpoint provided by this service.
*/

use expo_push_notification_client::{Expo, ExpoClientOptions, ExpoPushMessage, ExpoPushTicket};
use tokio::task::JoinSet;

const MAX_CONCURRENT_REQUESTS: usize = 200;

#[derive(serde::Deserialize)]
struct Notification {
    title: String,
    body: String,
    push_token: String,
}

pub async fn send_notifications(recipients_json: &str) -> Result<(), Box<dyn std::error::Error>> {
    let notifications: Vec<Notification> = serde_json::from_str(recipients_json)?;

    let total = notifications.len();

    if total == 0 {
        println!("No notifications to send.");
        return Ok(());
    }

    let expo = Expo::new(ExpoClientOptions { access_token: None });

    let mut tasks = JoinSet::new();

    let mut successful = 0usize;
    let mut failures = 0usize;

    for notification in notifications {
        let expo = expo.clone();

        tasks.spawn(async move {
            let message = match ExpoPushMessage::builder([notification.push_token.as_str()])
                .title(&notification.title)
                .body(&notification.body)
                .build()
            {
                Ok(message) => message,
                Err(error) => {
                    eprintln!("Message build error: {}", error);
                    return (0usize, 1usize);
                }
            };

            match expo.send_push_notifications(message).await {
                Ok(tickets) => {
                    let mut success = 0usize;
                    let mut failed = 0usize;

                    if tickets.is_empty() {
                        failed += 1;
                    }

                    for ticket in tickets {
                        match &ticket {
                            ExpoPushTicket::Ok(_) => {
                                success += 1;
                            }
                            ExpoPushTicket::Error(error) => {
                                eprintln!("Notification error: {}", error.message);
                                failed += 1;
                            }
                        }
                    }

                    (success, failed)
                }
                Err(error) => {
                    eprintln!("Request failed: {}", error);
                    (0, 1)
                }
            }
        });

        if tasks.len() >= MAX_CONCURRENT_REQUESTS {
            if let Some(result) = tasks.join_next().await {
                match &result {
                    Ok((s, f)) => {
                        successful += s;
                        failures += f;
                    }
                    Err(error) => {
                        eprintln!("Task failed: {}", error);
                        failures += 1;
                    }
                }
            }
        }
    }

    while let Some(result) = tasks.join_next().await {
        match &result {
            Ok((s, f)) => {
                successful += s;
                failures += f;
            }
            Err(error) => {
                eprintln!("Task failed: {}", error);
                failures += 1;
            }
        }
    }

    println!(
        "Finished sending: {}/{} successful - {} failures",
        &successful, &total, &failures
    );

    Ok(())
}
