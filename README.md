# Google Chat Types

type helper for construct Google Chat [message](https://developers.google.com/chat/api/guides/message-formats/basic)

## About Google Chat Message

There two type of Google Chat message

- Text Message
- Card Message

they are all represented as a json string.

Text Message represented like

```json
{
  "text": "some text"
}
```

Card Message represented like

```json
{
  "cards": [
    {
      "sections": [
        {
          "widgets": [
            {
              "image": { "imageUrl": "https://..." }
            },
            {
              "buttons": [
                {
                  "textButton": {
                    "text": "OPEN IN GOOGLE MAPS",
                    "onClick": {
                      "openLink": {
                        "url": "https://..."
                      }
                    }
                  }
                }
              ]
            }
          ]
        }
      ]
    }
  ]
}
```

the relationship between elements of cards should looks like below

<img src="/images/card_message.png" alt="Alt text" title="Optional title">

## How to use this crate

you should construct Cards or Text struct,
then serialize them to json string as a Google Chat API(for instance [incoming webhook](https://developers.google.com/chat/how-tos/webhooks)) http request body.
