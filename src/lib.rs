use async_trait::async_trait;

pub use errors::*;
pub mod akshare;
pub mod errors;
pub mod indicator;

#[async_trait]
pub trait ValueFactorService: Sized {
    type InitParams;
    type InnerClient;
    async fn new(param: Self::InitParams) -> Result<Self>;
    fn inner_client(&self) -> Option<&Self::InnerClient>;
}
