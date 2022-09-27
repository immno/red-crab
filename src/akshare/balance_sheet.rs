use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::errors::Result;

use super::{model::date_request::DateReq, Akshare};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct BalanceSheetData {
    #[serde(rename = "股票代码")]
    pub symbol: String,
    #[serde(rename = "股票简称")]
    pub name: String,
    #[serde(rename = "资产-货币资金")]
    pub zc_hbzj: Option<f64>,
    #[serde(rename = "资产-应收账款")]
    pub zc_yszk: Option<f64>,
    #[serde(rename = "资产-存货")]
    pub zc_ch: Option<f64>,
    #[serde(rename = "资产-总资产")]
    pub zc_zzc: Option<f64>,
    #[serde(rename = "资产-总资产同比")]
    pub zc_zzctb: Option<f64>,
    #[serde(rename = "负债-应付账款")]
    pub fz_yfzk: Option<f64>,
    #[serde(rename = "负债-总负债")]
    pub fz_zfz: Option<f64>,
    #[serde(rename = "负债-预收账款")]
    pub fz_yszk: Option<f64>,
    #[serde(rename = "负债-总负债同比")]
    pub fz_zfztb: Option<f64>,
    #[serde(rename = "资产负债率")]
    pub zcfzl: Option<f64>,
    #[serde(rename = "股东权益合计")]
    pub gdqyhj: Option<f64>,
}

impl Akshare {
    pub async fn get_balance_sheet(
        &self,
        q: &DateReq,
    ) -> Result<HashMap<String, BalanceSheetData>> {
        let data: Vec<BalanceSheetData> = self.transport.get("stock_zcfz_em", Some(q)).await?;
        let map = data
            .iter()
            .map(|x| (x.symbol.to_owned(), x.to_owned()))
            .collect::<HashMap<String, BalanceSheetData>>();
        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let akshare = Akshare::new();
        if let Ok(client) = akshare {
            let s = client.get_balance_sheet(&"20220331".into()).await.unwrap();
            // if let Ok(map) = s {
            print!("{:?}", s.get("000651"));
            // }
        }
    }
}
