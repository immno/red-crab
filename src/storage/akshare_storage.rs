use crate::errors::Result;
use crate::transport::Transport;

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
