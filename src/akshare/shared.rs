pub type Result<T> = std::result::Result<T, crate::errors::RedCrabError>;

pub mod string_to_decimal {
    use std::fmt;

    use rust_decimal::prelude::*;
    use serde::{de, Deserialize, Deserializer, Serializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringToDecimal {
            String(String),
        }

        let StringToDecimal::String(s) = StringToDecimal::deserialize(deserializer)?;
        Decimal::from_str(&s).map_err(de::Error::custom)
    }
}

pub mod string_to_opt_decimal {
    use rust_decimal::prelude::*;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Option<Decimal>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(value) = value {
            return serializer.collect_str(&value);
        }
        serializer.serialize_none()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringToOptDecimal {
            String(Option<String>),
        }
        let StringToOptDecimal::String(s) = StringToOptDecimal::deserialize(deserializer)?;
        if let Some(s) = s {
            return Decimal::from_str(&s).map(Some).or(Ok(None));
        }
        Ok(None)
    }
}

pub mod report_date_from_string {
    use std::fmt;

    use chrono::naive::NaiveDateTime;
    use serde::{Deserialize, Deserializer, Serializer};

    use crate::akshare::ReportDate;

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ReportDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let a = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S");

        match a {
            Ok(t) => Ok(t.date().into()),
            Err(e) => {
                let ss = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d")
                    .map_err(serde::de::Error::custom)?;
                Ok(ss.date().into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sh_list_works() {
        let date =
            chrono::NaiveDateTime::parse_from_str("2022-06-30 00:00:00", "%Y-%m-%d %H:%M:%S");
        println!("{:?}", date.unwrap().date().to_string());
        let s = "2022-06-30T00:00:00";
        let aa: chrono::NaiveDateTime = s.parse().unwrap();
        println!("{:?}", aa.date().to_string());
    }
}
