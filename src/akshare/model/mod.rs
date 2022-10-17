use std::fmt;

use serde::{Deserialize, Serialize};

pub mod date_request;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Quarter {
    Q1,
    Q2,
    Q3,
    Q4,
}

impl fmt::Display for Quarter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// impl From<&str> for Quarter {
//     fn from(s: &str) -> Self {
//         if s.contains("0331") {
//             Q1
//         } else if s.contains("0630") {
//             Q2
//         } else if s.contains("0930") {
//             Q3
//         } else if s.contains("1231") {
//             Q4
//         }
//     }
// }
