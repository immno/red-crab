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
            name: String::from("总资产"),
            title: String::from("了解公司的实力和成长性"),
            desc: String::from("大于等于10%，公司在扩张之中，成长性较好；0-10之间，公司处于成熟期，发展平稳；小于等于0，公司很可能处于收缩或者衰退之中。"),
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
        let yoy = data.1 / 100.0;
        if yoy >= 0.1 {
            Ok(80.0 + 20.0 * (yoy - 0.1))
        } else if yoy > 0.0 {
            Ok(40.0 + 20.0 * yoy)
        } else {
            Ok(0.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let step1 = FundamentalStep1(Akshare::new().unwrap());
        println!(
            "{:?}",
            step1
                .get_score(
                    "SZ002027",
                    &ReportDate::new(2022, crate::akshare::Quarter::Q1)
                )
                .await
                .unwrap()
        );
        println!(
            "{:?}",
            step1
                .get_data(
                    "SZ002027",
                    &ReportDate::new(2022, crate::akshare::Quarter::Q1)
                )
                .await
                .unwrap()
        );
    }
}
