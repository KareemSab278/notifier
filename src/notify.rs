use dotenv::dotenv;
use expo_push_notification_client::{Expo, ExpoClientOptions, ExpoPushMessage, ExpoPushTicket};
use std::env;

pub async fn send_test_notification() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    // temporary push token for testing purposes - will remove dotenv in prod since all tokens in db.
    let push_token = env::var("test_notification_token")?;


    let expo = Expo::new(ExpoClientOptions {
        access_token: env::var("EXPO_ACCESS_TOKEN").ok(),
    });

    let message = ExpoPushMessage::builder([push_token.as_str()])
        .title("Test notification")
        .body("This is a test notification.")
        .build()?;
    let tickets = expo.send_push_notifications(message).await?;

    for ticket in tickets {
        match ticket {
            ExpoPushTicket::Ok(ticket) => {
                println!("Test notification accepted (ticket {}).", ticket.id);
            }
            ExpoPushTicket::Error(error) => {
                return Err(std::io::Error::other(error.message).into());
            }
        }
    }

    Ok(())
}
