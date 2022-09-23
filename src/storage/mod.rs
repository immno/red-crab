pub mod balance_sheet;
pub mod akshare_storage;
pub mod model;

use std::fmt;

pub use crate::errors::*;
use crate::indicator::Indicator;
use async_trait::async_trait;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait FinancialIndicator {
    /// 获取数据
    async fn get(&self, symbole: &IndicatorSymbol) -> Result<Option<Decimal>>;
}

#[async_trait]
pub trait FinancialIndicatorFactory: FinancialIndicator {
    fn support(&self, indicator: &Indicator) -> Result<Option<Decimal>>;
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub struct IndicatorSymbol(pub String, pub Quarter);

/// to_symbol
impl ToString for IndicatorSymbol {
    fn to_string(&self) -> String {
        format!("{}[{}]", self.0, self.1).to_uppercase()
    }
}

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
