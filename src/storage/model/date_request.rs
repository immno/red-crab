use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DateReq {
    pub date: String,
}