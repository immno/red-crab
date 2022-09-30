use std::collections::HashMap;

use crate::{akshare::model::date_request::DateReq, errors::Result};
use async_trait::async_trait;

use crate::{akshare::Akshare, ValueFactorService};

use super::Descriptors;

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
        let akshare = Akshare::from(parameters.url)?;
        Ok(FundamentalFactor { client: akshare })
    }

    fn inner_client(&self) -> Option<&Self::InnerClient> {
        Some(&self.client)
    }
}

impl FundamentalFactor {
    // Latest price for ONE symbol.
    pub async fn step1_total_assets(&self, q: &DateReq) -> Result<HashMap<String, Descriptors>> {
        self.client.get_balance_sheet(q).await?.iter().map(|x| {
            let a1 = x.1.zc_zzc;
            let a2 = x.1.zc_zzctb;
        });
        todo!()
    }
}

struct Cache<T: Sized> {
    data: HashMap<String, HashMap<String, T>>,
}
