use async_trait::async_trait;

pub mod fundamental_analysis;
use crate::errors::Result;

#[async_trait]
pub trait ValueFactor {
    /// 获取基本信息
    fn get_info(&self) -> Descriptors;
    /// 获取评分
    async fn get_score(&self, symbol: &str, date: &str) -> Result<Vec<f32>>;
    /// 获取基础数据
    async fn get_data(&self, symbol: &str, date: &str) -> Result<Vec<f32>>;
}

pub struct Descriptors {
    pub title: String,
    pub desc: String,
}
