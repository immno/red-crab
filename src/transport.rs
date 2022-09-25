use reqwest::{header, Response, StatusCode, Url};
use serde::{de::DeserializeOwned, Serialize};

use crate::errors::{RedCrabError, Result};

#[derive(Clone, Debug)]
pub struct Transport {
    client: reqwest::Client,
    base_url: String,
}

impl Transport {
    pub fn new() -> Result<Self> {
        let url = String::from("http://127.0.0.1:8080/api/public/");
        Self::from(url)
    }

    pub fn from(url: String) -> Result<Self> {
        let default_headers = Self::default_headers();
        let client = reqwest::Client::builder()
            .default_headers(default_headers)
            .build()?;

        Ok(Transport {
            client,
            base_url: url,
        })
    }

    pub fn default_headers() -> header::HeaderMap<header::HeaderValue> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("red-crab"),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        headers
    }

    pub async fn get<O, S>(&self, endpoint: &str, params: Option<&S>) -> Result<O>
    where
        O: DeserializeOwned,
        S: Serialize,
    {
        let url = self.get_url(endpoint, params)?;
        let request = self.client.get(url).send().await?;

        self.response_handler(request).await
    }

    pub async fn post<O, D>(&self, endpoint: &str, data: Option<&D>) -> Result<O>
    where
        O: DeserializeOwned,
        D: Serialize,
    {
        let url = self.get_url::<()>(endpoint, None)?;
        let request = self.client.post(url).form(&data).send().await?;

        self.response_handler(request).await
    }

    pub fn get_url<Q>(&self, endpoint: &str, params: Option<&Q>) -> Result<Url>
    where
        Q: Serialize,
    {
        let url = format!("{}{}", self.base_url, endpoint);

        let mut url = Url::parse(&url)?;

        if params.is_some() {
            let query = serde_urlencoded::to_string(params)?;
            url.set_query(Some(&query));
        };

        Ok(url)
    }

    async fn response_handler<O>(&self, response: Response) -> Result<O>
    where
        O: DeserializeOwned,
    {
        match response.status() {
            StatusCode::OK => Ok(response.json::<O>().await?),
            StatusCode::INTERNAL_SERVER_ERROR => Err(RedCrabError::InternalServerError()),
            StatusCode::SERVICE_UNAVAILABLE => Err(RedCrabError::ServiceUnavailable()),
            StatusCode::UNAUTHORIZED => Err(RedCrabError::Unauthorized()),
            StatusCode::BAD_REQUEST => {
                let error = response.text().await?;

                Err(RedCrabError::InvalidParameter(error))
            }
            s => Err(RedCrabError::UnkownResponse(format!(
                "Received response: {:?}",
                s
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn it_works() {
        let t = Transport::new();
        if let Ok(tran) = t {
            let params = json! {{"date": "20220331"}};

            let url = tran.get_url("stock_zcfz_em", Some(&params)).unwrap();
            println!("{}", url.as_str());
            let request: String = tran
                .client
                .get(url)
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            println!("{:?}", request);
        }
    }
}
