use chrono::{DateTime, Duration, Local};
use indexmap::IndexSet;
use reqwest::{Error as ReqwestError, Response};
use rss::Channel;
use std::{error::Error, fmt};
use tokio::time::{error::Elapsed, timeout};

use crate::article::Article;

enum RequestOrTimeoutError {
    ReqwestError(ReqwestError),
    TimeoutError(Elapsed),
}

impl fmt::Display for RequestOrTimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestOrTimeoutError::ReqwestError(err) => write!(f, "Reqwest error: {}", err),
            RequestOrTimeoutError::TimeoutError(_) => write!(f, "Request timed out"),
        }
    }
}

pub struct ContentIngestion {
    feeds: Vec<String>,
    last_fetched: Option<DateTime<Local>>,
    all_unique_articles: IndexSet<Article>,
    should_update_last_fetched: bool,
    client: reqwest::Client,
}

impl ContentIngestion {
    pub fn new(feeds: Vec<String>) -> Self {
        Self {
            feeds,
            last_fetched: None,
            all_unique_articles: IndexSet::new(),
            should_update_last_fetched: true,
            client: reqwest::Client::new(),
        }
    }

    /// Fetches all unique articles.
    pub async fn fetch_all(&mut self) -> &IndexSet<Article> {
        if let Some(last_fetched) = self.last_fetched {
            if Local::now() - last_fetched < Duration::minutes(30) {
                return &self.all_unique_articles;
            }
        }

        for feed in &self.feeds.clone() {
            match self.fetch_newsarticles_from_feed(feed).await {
                Ok(Some(articles)) => {
                    self.process_new_articles(articles);
                }
                Ok(None) => {}
                Err(err) => {
                    // TODO: Handle this
                    eprintln!("Error: {}", err);
                }
            }
        }
        &self.all_unique_articles
    }

    ///Fetches news articles from a specific feed
    pub async fn fetch_newsarticles_from_feed(
        &mut self,
        feed: &str,
    ) -> Result<Option<Vec<Article>>, Box<dyn Error>> {
        let content = reqwest::get(feed).await?.bytes().await?;
        let channel = Channel::read_from(&content[..])?;

        let mut new_articles: Vec<Article> = vec![];
        for item in channel.items() {
            match DateTime::parse_from_rfc2822(item.pub_date().unwrap()) {
                Ok(parsed_timestamp) => {
                    match self.last_fetched {
                        Some(last_fetched) => {
                            if parsed_timestamp < last_fetched {
                                new_articles.push(Article::new(
                                    item.guid().unwrap().value.to_owned(),
                                    item.link().unwrap().to_owned(),
                                    None,
                                    None,
                                ));
                            }
                        }
                        //First time fetching in applications runtime 'self.last_fetched' will be None
                        //'self.last_fetched' is then set at the end.
                        None => {
                            new_articles.push(Article::new(
                                item.guid().unwrap().value.to_owned(),
                                item.link().unwrap().to_owned(),
                                None,
                                None,
                            ));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing RFC 2822 timestamp: {}", e);
                }
            }
        }

        self.update_last_fetched();

        if new_articles.is_empty() {
            Ok(None)
        } else {
            Ok(Some(new_articles))
        }
    }

    pub async fn scrape_articles(&mut self) -> Result<String, Box<dyn Error>> {
        //TODO: Associate the links with their response. Associate the links with their article. 
        //In this way we can associate the article with their response
        let links: Vec<&str> = self
            .all_unique_articles
            .iter()
            .map(|article| article.link())
            .collect();

        let responses = self.concurrent_fetch_multiple_urls(&links).await;

        for response in responses {
            match response {
                Ok(response) => {
                    match response.text().await {
                        Ok(html) => {
                            let _document = scraper::Html::parse_document(&html);
                            println!("Parsed the html response")
                        }
                        Err(err) => {
                            // Handle error when retrieving text
                            println!("Error reading response: {}", err);
                        }
                    }
                }
                Err(error) => {
                    // TODO: Handle error in fetching URL. The article should be retried i guess.
                    println!("Error fetching URL: {}", error);
                }
            }
        }
        println!("done");

        Ok("123".to_owned())
    }

    async fn concurrent_fetch_multiple_urls(
        &self,
        urls: &[&str],
    ) -> Vec<Result<reqwest::Response, RequestOrTimeoutError>> {
        //TODO: implementer chunking. Måske implementer det i scrape_articles
        let futures = urls.iter().map(|&url| self.fetch_url(url));
        let results = futures::future::join_all(futures).await;
        println!("{}", results.len());
        results
    }

    async fn fetch_url(&self, url: &str) -> Result<Response, RequestOrTimeoutError> {
        let timeout_duration = std::time::Duration::from_secs(10);
        let response = timeout(timeout_duration, self.client.get(url).send()).await;
        match response {
            Ok(result) => match result {
                Ok(res) => Ok(res),
                Err(err) => Err(RequestOrTimeoutError::ReqwestError(err)),
            },
            Err(error) => Err(RequestOrTimeoutError::TimeoutError(error)),
        }
    }

    fn process_new_articles(&mut self, articles: Vec<Article>) {
        self.all_unique_articles.extend(articles)
    }

    fn update_last_fetched(&mut self) {
        //TODO: Implemter den delta tid der skal være før vi prøver at fetch igen.
        if self.should_update_last_fetched {
            self.last_fetched = Some(Local::now());
            self.should_update_last_fetched = false;
        }
    }
}
