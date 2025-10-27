use crate::ping_error::PingError;
use log::trace;
use reqwest::Client;
use reqwest::Method;
use std::str::FromStr;
use std::time::Duration;
use wheel_rs::urn_utils::Urn;

#[derive(Clone)]
pub struct HttpPing {
    client: Client,
    method: Method,
    url: String,
}
impl HttpPing {
    pub fn new(urn: String) -> Self {
        let urn = Urn::new(urn);
        HttpPing {
            client: Client::builder().build().unwrap(),
            method: Method::from_str(urn.method.to_string().as_str()).unwrap(),
            url: urn.url,
        }
    }

    pub async fn ping(&self, timeout: Duration) -> Result<(), PingError> {
        trace!("ping {}:{} ....", self.method, self.url);

        // 发出Http请求，并判断返回状态是否是200
        let response = self
            .client
            .request(self.method.clone(), &self.url)
            .timeout(timeout)
            .send()
            .await?;

        if response.status().is_success() {
            trace!("ping {}:{} success", self.method, self.url);
            Ok(())
        } else {
            Err(PingError::InvalidReply(format!(
                "HTTP request failed with status: {}",
                response.status()
            )))
        }
    }
}
