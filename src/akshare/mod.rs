pub mod balance_sheet;
pub mod cash_flow_statement;
pub mod income_statement;
pub mod model;
mod transport;

use crate::errors::Result;

use self::transport::Transport;

#[derive(Clone)]
pub struct Akshare {
    pub transport: Transport,
}

impl Akshare {
    pub fn new() -> Result<Self> {
        let t = Transport::new()?;
        Ok(Self { transport: t })
    }

    pub fn from(url: String) -> Result<Self> {
        let t = Transport::from(url)?;
        Ok(Self { transport: t })
    }
}
