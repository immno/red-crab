use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::errors::Result;

use super::{model::date_request::DateReq, Akshare};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct IncomeStatementData {
    #[serde(rename = "股票代码")]
    pub symbol: String,
    #[serde(rename = "股票简称")]
    pub name: String,
    #[serde(rename = "净利润")]
    pub jlr: Option<f64>,
    #[serde(rename = "净利润同比")]
    pub jlrtb: Option<f64>,
    #[serde(rename = "营业总收入")]
    pub yyzsr: Option<f64>,
    #[serde(rename = "营业总收入同比")]
    pub yyzsrtb: Option<f64>,
    #[serde(rename = "营业总支出-营业支出")]
    pub yyzzc_yyzc: Option<f64>,
    #[serde(rename = "营业总支出-销售费用")]
    pub yyzzc_xsfy: Option<f64>,
    #[serde(rename = "营业总支出-管理费用")]
    pub yyzzc_glfy: Option<f64>,
    #[serde(rename = "营业总支出-财务费用")]
    pub yyzzc_cwfy: Option<f64>,
    #[serde(rename = "营业总支出-营业总支出")]
    pub yyzzc_yyzzc: Option<f64>,
    #[serde(rename = "营业利润")]
    pub ttlr: Option<f64>,
    #[serde(rename = "利润总额")]
    pub lrze: Option<f64>,
}

impl Akshare {
    pub async fn get_income_statement(
        &self,
        q: &DateReq,
    ) -> Result<HashMap<String, IncomeStatementData>> {
        let data: Vec<IncomeStatementData> = self.transport.get("stock_lrb_em", Some(q)).await?;
        let map = data
            .iter()
            .map(|x| (x.symbol.to_owned(), x.to_owned()))
            .collect::<HashMap<String, IncomeStatementData>>();
        Ok(map)
    }
}
