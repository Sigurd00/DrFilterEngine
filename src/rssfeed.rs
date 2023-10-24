use std::error::Error;
use rss::Channel;

pub async fn get_feed(feed: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(feed)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

//TODO: Change name of this function
pub fn find_content(channel: Channel, keywords: Vec<String>) -> Result<i32, String> {
    for item in channel.items() {
        if let Some(text) = item.description() {
            for keyword in &keywords {
                if text.contains(keyword) {
                    return Ok(123)
                }
            }
        }
    }
    Err("didnt find shit".to_string())
}
