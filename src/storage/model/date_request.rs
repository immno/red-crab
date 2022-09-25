use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DateReq {
    pub date: String,
}

impl From<&str> for DateReq {
    fn from(s: &str) -> Self {
        Self { date: s.to_owned() }
    }
}
