use chrono::{DateTime, Duration, Local};
use indexmap::IndexSet;
use rss::Channel;
use std::error::Error;

pub struct ContentIngestion {
    feeds: Vec<String>,
    last_fetched: Option<DateTime<Local>>,
    all_unique_articles: IndexSet<String>,
    should_update_last_fetched: bool,
}

impl ContentIngestion {
    pub fn new(feeds: Vec<String>) -> Self {
        Self {
            feeds,
            last_fetched: None,
            all_unique_articles: IndexSet::new(),
            should_update_last_fetched: true,
        }
    }

    /// Fetches all unique articles.
    pub async fn fetch_all(&mut self) -> &IndexSet<String> {
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
    ) -> Result<Option<Vec<String>>, Box<dyn Error>> {
        let content = reqwest::get(feed).await?.bytes().await?;
        let channel = Channel::read_from(&content[..])?;

        let mut new_articles: Vec<String> = vec![];
        for item in channel.items() {
            match DateTime::parse_from_rfc2822(item.pub_date().unwrap()) {
                Ok(parsed_timestamp) => {
                    match self.last_fetched {
                        Some(last_fetched) => {
                            if parsed_timestamp < last_fetched {
                                new_articles.push(item.link().unwrap().to_owned());
                            }
                        }
                        //First time fetching in applications runtime 'self.last_fetched' will be None
                        //'self.last_fetched' is then set at the end.
                        None => {
                            new_articles.push(item.link().unwrap().to_owned());
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

    fn process_new_articles(&mut self, articles: Vec<String>) {
        self.all_unique_articles.extend(articles);
    }

    fn update_last_fetched(&mut self) {
        //TODO: Implemter den delta tid der skal være før vi prøver at fetch igen.
        if self.should_update_last_fetched {
            self.last_fetched = Some(Local::now());
            self.should_update_last_fetched = false;
        }
    }
}
