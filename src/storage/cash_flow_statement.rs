use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::errors::Result;

use super::{akshare_storage::Akshare, model::date_request::DateReq};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CashFlowStatementData {
    #[serde(rename = "股票代码")]
    pub symbol: String,
    #[serde(rename = "股票简称")]
    pub name: String,
    #[serde(rename = "净现金流-净现金流")]
    pub jxjl_jxjl: Option<f64>,
    #[serde(rename = "净现金流-同比增长")]
    pub jxjl_tbzz: Option<f64>,
    #[serde(rename = "经营性现金流-现金流量净额")]
    pub jyxxjl_xjllje: Option<f64>,
    #[serde(rename = "经营性现金流-净现金流占比")]
    pub jyxxjl_jxjlzb: Option<f64>,
    #[serde(rename = "投资性现金流-现金流量净额")]
    pub tzxxjl_xjllje: Option<f64>,
    #[serde(rename = "投资性现金流-净现金流占比")]
    pub tzxxjl_jxjlzb: Option<f64>,
    #[serde(rename = "融资性现金流-现金流量净额")]
    pub rzxxjl_xjllje: Option<f64>,
    #[serde(rename = "融资性现金流-净现金流占比")]
    pub rzxxjl_jxjlzb: Option<f64>,
}

impl Akshare {
    pub async fn get_cash_flow_statement(
        &self,
        q: &DateReq,
    ) -> Result<HashMap<String, CashFlowStatementData>> {
        let data: Vec<CashFlowStatementData> = self.transport.get("stock_xjll_em", Some(q)).await?;
        let map = data
            .iter()
            .map(|x| (x.symbol.to_owned(), x.to_owned()))
            .collect::<HashMap<String, CashFlowStatementData>>();
        Ok(map)
    }
}
