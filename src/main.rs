use dr_filter_engine::rssfeed::get_feed;
use futures::future::join_all;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let feeds:Vec<&str> = vec![
        "https://www.dr.dk/nyheder/service/feeds/senestenyt",
        "https://www.dr.dk/nyheder/service/feeds/indland",
        "https://www.dr.dk/nyheder/service/feeds/udland",
        "https://www.dr.dk/nyheder/service/feeds/penge",
        "https://www.dr.dk/nyheder/service/feeds/politik",
        "https://www.dr.dk/nyheder/service/feeds/sporten",
        "https://www.dr.dk/nyheder/service/feeds/senestesport",
        "https://www.dr.dk/nyheder/service/feeds/viden",
        "https://www.dr.dk/nyheder/service/feeds/kultur",
        "https://www.dr.dk/nyheder/service/feeds/musik",
        "https://www.dr.dk/nyheder/service/feeds/mitliv",
        "https://www.dr.dk/nyheder/service/feeds/mad",
        "https://www.dr.dk/nyheder/service/feeds/vejret",
        "https://www.dr.dk/nyheder/service/feeds/regionale",
        "https://www.dr.dk/nyheder/service/feeds/regionale/kbh",
        "https://www.dr.dk/nyheder/service/feeds/regionale/bornholm",
        "https://www.dr.dk/nyheder/service/feeds/regionale/syd",
        "https://www.dr.dk/nyheder/service/feeds/regionale/fyn",
        "https://www.dr.dk/nyheder/service/feeds/regionale/vest",
        "https://www.dr.dk/nyheder/service/feeds/regionale/nord",
        "https://www.dr.dk/nyheder/service/feeds/regionale/trekanten",
        "https://www.dr.dk/nyheder/service/feeds/regionale/sjaelland",
        "https://www.dr.dk/nyheder/service/feeds/regionale/oestjylland"
    ];
    let mut contents = vec![];
    for feed in feeds {
        contents.push(get_feed(feed));
    }
    let channels= join_all(contents).await;
    for channel in channels {
        if let Ok(channel) = channel {
            
        }
    }
}
