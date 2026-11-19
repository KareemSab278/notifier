# notifier

A small HTTP service that sends push notifications through the Expo Push Service.

## Run

Start the server with:

```sh
cargo run
```

The server listens on port `6767` on all network interfaces. Send requests to
`http://localhost:6767` when calling it from the same machine.

## How it works

Send a `POST` request to `/send_notifications` (a trailing slash is also
accepted) with a JSON array. Each array item is one notification and must
contain non-empty `title`, `body`, and `push_token` strings:

```json
[
  {
    "title": "Hello",
    "body": "This is a test notification.",
    "push_token": "ExponentPushToken[YOUR_EXPO_PUSH_TOKEN]"
  }
]
```

The service validates the JSON and required values, then sends the notifications
to Expo in sequential batches of up to 100. The response includes the count of
successful Expo tickets and `failed`, the zero-based indexes of notifications
that failed in the original array. A `success: true` response means the request
was processed; check `failed` to see whether some notifications failed. An
accepted Expo ticket does not guarantee that the device displayed the
notification.

For example, if the first notification succeeded and the second failed:

```json
{
  "success": true,
  "message": "Sent 1 notification(s); 1 failed",
  "failed": [1]
}
```

Invalid JSON or a missing/empty required value returns HTTP `400` with an
`error` message. A server-side failure returns HTTP `500`.

## Test with curl

Replace the example token with a valid Expo push token. This sends a real
notification.

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
