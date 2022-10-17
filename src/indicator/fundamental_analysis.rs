use std::collections::HashMap;

use crate::errors::Result;
use async_trait::async_trait;

use crate::{akshare::Akshare, ValueFactorService};

pub struct Parameters {
    pub url: String,
}

#[derive(Clone)]
pub struct FundamentalFactor {
    pub client: Akshare,
}

#[async_trait]
impl ValueFactorService for FundamentalFactor {
    type InitParams = Parameters;
    type InnerClient = Akshare;

    async fn new(parameters: Self::InitParams) -> Result<Self> {
        let akshare = Akshare::from(parameters.url.as_str())?;
        Ok(FundamentalFactor { client: akshare })
    }

    fn inner_client(&self) -> Option<&Self::InnerClient> {
        Some(&self.client)
    }
}

struct Cache<T: Sized> {
    data: HashMap<String, HashMap<String, T>>,
}
