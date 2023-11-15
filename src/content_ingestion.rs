use chrono::{DateTime, Duration, Local};
use indexmap::IndexSet;
use rss::Channel;
use std::error::Error;

use crate::article::Article;

pub struct ContentIngestion {
    feeds: Vec<String>,
    last_fetched: Option<DateTime<Local>>,
    all_unique_articles: IndexSet<Article>,
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
        let response = reqwest::get("https://scrapeme.live/shop/").await;
        let html_content = response.unwrap().text().await.unwrap();
        let document = scraper::Html::parse_document(&html_content);

        //TODO: Find out how to consistently scrape articles for their content
        Ok("asdasd".to_string())
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

async fn concurrent_fetch_multiple_urls(urls: &[&str])  {
    let futures = urls.iter().map(|&url| fetch_url(url));
    let results = futures::future::join_all(futures).await;

    for result in results {
        if let Err(e) = result {
            eprintln!("Error: {:?}", e);
        }
    }
}

async fn fetch_url(url: &str) -> Result<(), reqwest::Error> {
    let response = reqwest::get(url).await?;
    // Process response here, for example, print the status code
    println!("Status for {}: {}", url, response.status());
    Ok(())
}
