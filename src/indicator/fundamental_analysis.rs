use crate::errors::Result;
use async_trait::async_trait;

use crate::{akshare::Akshare, ValueFactor};

pub struct Parameters {
    pub url: String,
}

#[derive(Clone)]
pub struct FundamentalFactor {
    pub client: Akshare,
}

#[async_trait]
impl ValueFactor for FundamentalFactor {
    type InitParams = Parameters;
    type InnerClient = Akshare;

    async fn new(parameters: Self::InitParams) -> Result<Self> {
        let akshare = Akshare::from(parameters.url)?;
        Ok(FundamentalFactor { client: akshare })
    }

    fn inner_client(&self) -> Option<&Self::InnerClient> {
        Some(&self.client)
    }
}

impl FundamentalFactor {}

pub struct Descriptors {
    pub name: String,
    pub score: f64,
    // pub info:
}
