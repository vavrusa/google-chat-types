//! Google Chat Types
//!
//! ## About Google Chat Message
//!
//! type helper for construct Google Chat [message](https://developers.google.com/chat/api/guides/message-formats/basic)
//! There two type of Google Chat message
//! - Text Message
//! - Card Message
//!
//! they are all represented as a json string.
//!  
//! Text Message represented like
//! ```json
//! {
//!     "text":"some text"
//! }  
//!```
//! Card Message represented like
//!```json
//!{
//! "cards": [
//!    {
//!      "sections": [
//!        {
//!          "widgets": [
//!            {
//!              "image": { "imageUrl": "https://..." }
//!            },
//!            {
//!              "buttons": [
//!                {
//!                  "textButton": {
//!                    "text": "OPEN IN GOOGLE MAPS",
//!                    "onClick": {
//!                      "openLink": {
//!                        "url": "https://..."
//!                      }
//!                    }
//!                  }
//!                }
//!              ]
//!            }
//!          ]
//!        }
//!      ]
//!    }
//!  ]
//!}
//! ```
//!
//! the relationship between elements of cards should looks like below
//!
//! <img src="https://future-architect.github.io/images/20210913a/screenshot_card_message.png" width="900px"></img>
//!
//! ## How to use this crate
//!
//! you should construct Cards or Text struct,
//! then serialize them to json string as a Google Chat API(for instance [incoming webhook](https://developers.google.com/chat/how-tos/webhooks)) http request body.

use std::fmt::Display;

use derive_builder::{Builder, UninitializedFieldError};
use serde::Serialize;
use thiserror::Error;

#[doc = "The Text message type"]
#[derive(Serialize, Clone, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(build_fn(error = "ChatTypeBuildError"))]
pub struct Text {
    #[builder(setter(into))]
    text: String,
}

#[doc = "The Card message type"]
#[derive(Serialize, Clone, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(build_fn(error = "ChatTypeBuildError"))]
pub struct Cards {
    cards: Vec<Card>,
}

/// the Card response.
/// construct this by call default() method of this type
#[derive(Serialize, Clone, Default, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(build_fn(error = "ChatTypeBuildError"))]
pub struct Card {
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<Header>,
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    sections: Option<Vec<Section>>,
}

#[derive(Serialize, Clone, Default, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(build_fn(error = "ChatTypeBuildError"))]
pub struct Header {
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    subtitle: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    image_style: Option<String>,
}

#[derive(Serialize, Clone, Default, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(build_fn(error = "ChatTypeBuildError"))]
pub struct Section {
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    header: Option<String>,
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    widgets: Option<Vec<Widget>>,
}

#[derive(Serialize, Clone, Default, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(build_fn(error = "ChatTypeBuildError"))]
pub struct Widget {
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    text_paragraph: Option<TextParagraph>,
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    key_value: Option<KeyValue>,
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<Image>,
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    buttons: Option<Vec<Button>>,
}

#[derive(Serialize, Clone, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(build_fn(error = "ChatTypeBuildError"))]
pub struct TextParagraph {
    #[builder(setter(into))]
    text: String,
}

#[derive(Serialize, Clone, Default, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(build_fn(error = "ChatTypeBuildError"))]
pub struct KeyValue {
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    top_label: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    content_multiline: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    bottom_label: Option<String>,
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    on_click: Option<OnClick>,
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    button: Option<Button>,
}

impl KeyValue {
    pub fn to_widget(&self) -> Widget {
        Widget {
            text_paragraph: None,
            image: None,
            buttons: None,
            key_value: Some(self.to_owned()),
        }
    }
}

#[derive(Serialize, Clone, Default, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(build_fn(error = "ChatTypeBuildError"))]
pub struct Image {
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<String>,
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    on_click: Option<OnClick>,
}

impl Image {
    pub fn to_widget(&self) -> Widget {
        Widget {
            text_paragraph: None,
            key_value: None,
            image: Some(self.to_owned()),
            buttons: None,
        }
    }
}

#[derive(Serialize, Clone, Default, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(build_fn(error = "ChatTypeBuildError"))]
pub struct Button {
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    text_button: Option<TextButton>,
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    image_button: Option<ImageButton>,
}

#[derive(Serialize, Clone, Default, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(build_fn(error = "ChatTypeBuildError"))]
pub struct TextButton {
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    on_click: Option<OnClick>,
}

#[derive(Serialize, Clone, Default, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(build_fn(error = "ChatTypeBuildError"))]
pub struct ImageButton {
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    icon_url: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<String>,
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    on_click: Option<OnClick>,
}

#[derive(Serialize, Clone, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(build_fn(error = "ChatTypeBuildError"))]
pub struct OnClick {
    #[builder(setter(into))]
    open_link: OpenLink,
}

#[derive(Serialize, Clone, Builder, Debug)]
#[serde(rename_all = "camelCase")]
#[builder(build_fn(error = "ChatTypeBuildError"))]
pub struct OpenLink {
    #[builder(setter(into))]
    url: String,
}

#[derive(Debug, Error)]
pub struct ChatTypeBuildError(String);

impl Display for ChatTypeBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Error in build process {}", self.0))
    }
}

impl From<UninitializedFieldError> for ChatTypeBuildError {
    fn from(e: UninitializedFieldError) -> Self {
        Self(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn build_cards() {
        let text_paragraph = TextParagraphBuilder::default()
            .text("some text")
            .build()
            .unwrap();
        let widget = WidgetBuilder::default()
            .text_paragraph(text_paragraph)
            .build()
            .unwrap();
        let section = SectionBuilder::default()
            .widgets(vec![widget])
            .build()
            .unwrap();
        let header = HeaderBuilder::default().title("some tile").build().unwrap();
        let card = CardBuilder::default()
            .sections(vec![section])
            .header(header)
            .build()
            .unwrap();
        let cards = CardsBuilder::default().cards(vec![card]).build().unwrap();
        let json = serde_json::json!(cards).to_string();
        assert_eq!(json,"{\"cards\":[{\"header\":{\"title\":\"some tile\"},\"sections\":[{\"widgets\":[{\"textParagraph\":{\"text\":\"some text\"}}]}]}]}")
    }

    #[test]
    fn build_text() {
        let text = TextBuilder::default().text("some text").build().unwrap();
        let json = serde_json::json!(text).to_string();
        assert_eq!(json, "{\"text\":\"some text\"}")
    }
}
