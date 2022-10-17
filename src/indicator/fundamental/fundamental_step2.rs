use async_trait::async_trait;

use crate::akshare::ReportDate;
use crate::errors::Result;
use crate::{
    akshare::Akshare,
    indicator::{Descriptors, ValueFactor},
};

#[derive(Clone)]
pub struct FundamentalStep1(Akshare);

#[async_trait]
impl ValueFactor for FundamentalStep1 {
    type Data = (f64, f64);

    fn get_info(&self) -> Descriptors {
        Descriptors {
            name: String::from("负债"),
            title: String::from("了解公司的偿债风险"),
            desc: String::from("大于等于70%，发生债务危机的可能性较大；大于等于40%，小于70%，偿债风险较小，但在特殊情况下依然可能发生偿债危机；小于40%，基本没有偿债风险。"),
            exp: String::from("总资产的同比增长率= (本年资产总计-上年资产总计)/上年资产总计"),
        }
    }

    async fn get_data(&self, symbol: &str, date: &ReportDate) -> Result<Self::Data> {
        let data = self
            .0
            .get_balance_sheet(&symbol.into())
            .await?
            .get(date)
            .map_or((f64::default(), f64::default()), |x| {
                (
                    x.total_assets.unwrap_or_default(),
                    x.total_assets_yoy.unwrap_or_default(),
                )
            });
        Ok(data)
    }

    async fn get_score(&self, symbol: &str, date: &ReportDate) -> Result<f64> {
        let data = self.get_data(symbol, date).await?;
        if data.1 >= 0.1 {
            Ok(80.0 + 20.0 * (data.1 - 0.1))
        } else if data.1 > 0.0 {
            Ok(40.0 + 20.0 * data.1)
        } else {
            Ok(0.0)
        }
    }
}
