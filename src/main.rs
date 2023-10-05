use dr_filter_engine::rssfeed::get_feed;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let content = get_feed("https://www.dr.dk/nyheder/service/feeds/senestenyt").await.unwrap();
    println!("{}", content.title);
}
