use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ThankYouMessage {
    pub program: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}
