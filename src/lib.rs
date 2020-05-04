extern crate tokio;
extern crate serde_json;

mod key;
mod client;
mod quotes;
mod price_history;
mod accounts;
mod link_handler;

// use crate::{
//     quotes::quotes::Quote,
//     price_history::price_history::PriceHistory
// };

#[cfg(test)]
mod tests {
    use crate::{
        quotes::quotes,
        price_history::price_history,
        serde_json::json,
    };
    use std::path::Path;
    use std::fs::File;
    use std::io::Write;
    use std::error::Error;
    use crate::accounts;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn titties() {
        println!("{:?}", quotes::get_quote("SPY").await);
    }

    #[tokio::test]
    async fn titties2() {
        let options = vec![
            ("periodType", "day"),
            ("period", "10"),
            ("frequencyType", "minute"),
            ("frequency", "10")
        ];

        let s: String = price_history::get_price_history("SPY", &options).await;
        println!("{:?}", s);

        let path = Path::new("test.txt");
        let mut file = match File::create(&path) {
            Err(e) => panic!("{}", e),
            Ok(file) => file
        };

        match file.write_all(s.as_bytes()) {
            Err(e) => eprintln!("{}", e),
            Ok(_) => println!("successfully wrote to {}", path.display()),
        };

        let p: price_history::PriceHistoryData = price_history::get_price_history_json("SPY", &options).await;
    }
}
