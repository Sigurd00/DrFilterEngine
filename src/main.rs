use dr_filter_engine::content_ingestion::ContentIngestion;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let mut ingestion = ContentIngestion::new(vec![
        "https://www.dr.dk/nyheder/service/feeds/senestenyt".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/indland".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/udland".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/penge".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/politik".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/sporten".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/senestesport".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/viden".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/kultur".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/musik".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/mitliv".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/mad".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/vejret".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/regionale".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/regionale/kbh".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/regionale/bornholm".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/regionale/syd".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/regionale/fyn".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/regionale/vest".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/regionale/nord".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/regionale/trekanten".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/regionale/sjaelland".to_string(),
        "https://www.dr.dk/nyheder/service/feeds/regionale/oestjylland".to_string(),
    ]);

    let _new_articles = ingestion.fetch_all().await;
    println!("Found {} unique articles", _new_articles.len());

    let _ = ingestion.scrape_articles().await;
}
