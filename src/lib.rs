use async_trait::async_trait;

pub use errors::*;
pub mod errors;
pub mod factor;
pub mod storage;
pub mod indicator;
pub mod transport;

#[async_trait]
pub trait ValueFactor: Sized {
    type InitParams;
    type InnerClient;
    async fn new(param: Self::InitParams) -> Result<Self>;
    fn inner_client(&self) -> Option<Self::InnerClient>;
}

#[async_trait]
pub trait BaseFactor {
    /// 名称
    fn get_name(&self) -> String;
    async fn score(&self) -> u64;
    async fn info(&self) -> Descriptors;
}

pub struct Descriptors {}