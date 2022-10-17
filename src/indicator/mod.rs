use async_trait::async_trait;

pub mod fundamental;
pub mod fundamental_analysis;
use crate::{akshare::ReportDate, errors::Result};

#[async_trait]
pub trait ValueFactor {
    type Data;

    /// 获取基本信息
    fn get_info(&self) -> Descriptors;
    /// 获取基础数据
    async fn get_data(&self, symbol: &str, date: &ReportDate) -> Result<Self::Data>;
    /// 获取评分
    async fn get_score(&self, symbol: &str, date: &ReportDate) -> Result<f64>;
}

pub struct Descriptors {
    pub name: String,
    pub title: String,
    pub desc: String,
    pub exp: String,
}
