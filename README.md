# notifier

A small HTTP service that sends push notifications through the Expo Push Service.

## Run

Start the server with:

```sh
cargo run
```

It listens on port `6767`.

## How it works

Send a `POST` request to `/send_notifications` with a JSON array of notifications. The service forwards each notification to Expo and returns a JSON result. Partial notification failures still return success; the response message reports the number sent and failed. Invalid JSON or a notification missing a required field returns a `400`.

Each notification in the array has a title, body, and Expo push token:

```json
[
  {
    "title": "Hello",
    "body": "This is a test notification.",
    "push_token": "ExponentPushToken[YOUR_EXPO_PUSH_TOKEN]"
  }
]
```

## Test with curl

Replace the example token with a valid Expo push token. This sends a real notification.

```sh
curl -X POST http://localhost:6767/send_notifications \
  -H "Content-Type: application/json" \
  -d '[
    {
      "title": "Hello",
      "body": "This is a test notification.",
      "push_token": "ExponentPushToken[YOUR_EXPO_PUSH_TOKEN]"
    }
  ]'
```
example response:

{"success":true,"message":"Sent 12 notification(s); 12 failed"}
