use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, Debug)]
pub struct ThankYouMessage {
    #[validate(
        length(min = 1, message = "Input needs to be at least one character long"),
        length(
            max = 50,
            message = "Tool name can't be longer than 50 characters, sorry!"
        )
    )]
    pub program: String,

    #[validate(length(
        max = 2048,
        message = "Note too long! Please keep it under 2048 characters."
    ))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThankYouStats {
    pub program: String,
    pub count: i64,
    pub note_count: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThankYouDetail {
    pub program: String,
    pub notes: Vec<String>,
}

#[test]
fn program_length_good() {
    let ty_message = ThankYouMessage {
        program: "good".to_string(),
        note: None,
    };

    assert!(ty_message.validate().is_ok())
}

#[test]
fn program_length_too_long() {
    let ty_message = ThankYouMessage {
        program: "Is this way too long or maybe just one character to long?".to_string(),
        note: None,
    };
    assert!(ty_message.validate().is_err())
}

#[test]
fn note_too_long() {
    let ty_message = ThankYouMessage {
        program: "good".to_string(),
        note: Some(include_str!("bad_note").to_string()),
    };
    assert!(ty_message.validate().is_err())
}

#[test]
fn note_unicode_good() {
    let ty_message = ThankYouMessage {
        program: "good".to_string(),
        note: Some(include_str!("good_emoji_note").to_string()),
    };
    assert!(ty_message.validate().is_ok())
}

#[test]
fn note_unicode_long() {
    let ty_message = ThankYouMessage {
        program: "good".to_string(),
        note: Some(include_str!("bad_emoji_note").to_string()),
    };
    assert!(ty_message.validate().is_err())
}

#[test]
fn note_length_good() {
    let ty_message = ThankYouMessage {
        program: "good".to_string(),
        note: Some(include_str!("good_note").to_string()),
    };
    assert!(ty_message.validate().is_ok())
}
