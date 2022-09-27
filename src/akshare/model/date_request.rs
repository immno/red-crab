use serde::{Deserialize, Serialize};

use super::Quarter;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DateReq {
    pub date: String,
}

impl DateReq {
    fn from_str(year: &str, q: &Quarter) -> DateReq {
        match q {
            Quarter::Q1 => Self::from_q1(year),
            Quarter::Q2 => Self::from_q2(year),
            Quarter::Q3 => Self::from_q3(year),
            Quarter::Q4 => Self::from_q4(year),
        }
    }

    fn from_q1(year: &str) -> DateReq {
        format!("{}{}", year, "0331").as_str().into()
    }

    fn from_q2(year: &str) -> DateReq {
        format!("{}{}", year, "0630").as_str().into()
    }

    fn from_q3(year: &str) -> DateReq {
        format!("{}{}", year, "0930").as_str().into()
    }

    fn from_q4(year: &str) -> DateReq {
        format!("{}{}", year, "1231").as_str().into()
    }
}

impl From<&str> for DateReq {
    fn from(s: &str) -> Self {
        Self { date: s.to_owned() }
    }
}
