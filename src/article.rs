use std::hash::{Hash, Hasher};

use reqwest::Response;
use tokio::time::timeout;

use crate::content_ingestion::RequestOrTimeoutError;

pub struct Article {
    uuid: String,
    link: String,
    content: Option<String>,
    identified_keywords: Option<Vec<String>>,
}

impl Article {
    pub fn new(
        uuid: String,
        link: String,
        content: Option<String>,
        identified_keywords: Option<Vec<String>>,
    ) -> Self {
        Self {
            uuid,
            link,
            content,
            identified_keywords,
        }
    }
    pub fn link(&self) -> &str {
        &self.link
    }

    pub fn content(&self) -> Option<&String> {
        self.content.as_ref()
    }

    pub fn identified_keywords(&self) -> Option<&Vec<String>> {
        self.identified_keywords.as_ref()
    }

    pub async fn fetch_url(&self, client: &reqwest::Client) -> Result<Response, RequestOrTimeoutError> {
        let timeout_duration = std::time::Duration::from_secs(10);
        let response = timeout(timeout_duration, client.get(url).send()).await;
        match response {
            Ok(result) => match result {
                Ok(res) => Ok(res),
                Err(err) => Err(RequestOrTimeoutError::ReqwestError(err)),
            },
            Err(error) => Err(RequestOrTimeoutError::TimeoutError(error)),
        }
    }
}

impl Eq for Article {}

impl PartialEq for Article {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Hash for Article {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state)
    }
}
