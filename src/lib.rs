extern crate tokio;
extern crate serde_json;

mod key;
mod client;
mod quotes;
mod price_history;
mod accounts;
mod link_handler;
mod option_chains;

// use crate::{
//     quotes::quotes::Quote,
//     price_history::price_history::PriceHistory
// };

#[cfg(test)]
mod tests {
    use crate::{quotes::quotes, price_history::price_history, serde_json::json, price_history::ml_testing, option_chains};
    use std::path::Path;
    use std::fs::File;
    use std::io::Write;
    use std::error::Error;
    use crate::accounts;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn titties() {
        let s = quotes::get_quote("SPY").await;
        let path = Path::new("test.json");
        let mut file = match File::create(&path) {
            Err(e) => panic!("{}", e),
            Ok(file) => file
        };

        match file.write_all(s.as_bytes()) {
            Err(e) => eprintln!("{}", e),
            Ok(_) => println!("successfully wrote to {}", path.display()),
        };
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

        let path = Path::new("test.json");
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

    #[tokio::test]
    async fn titties3() {
        let mut options: Vec<(&str, &str)> = vec![
            ("symbol", "SPY"),
            ("contractType", "CALL"),
            ("strikeCount", "1"),
            ("includeQuotes", "TRUE"),
            ("strategy", "SINGLE"),
            ("range", "ALL"),
            ("fromDate", "2020-03-01"),
            ("toDate", "2020-06-05"),
            ("expMonth", "ALL"),
            ("optionType", "ALL")
        ];

        let value = option_chains::get_option_chain(&options).await;
        println!("{:#?}", value.call_exp_date_map);

        for i in value.call_exp_date_map {
            for j in i.1 {
                for k in j.1 {
                    println!("{:?}", k);
                    // for (a, b) in k.iter() {
                    //     println!("{:?} : {:?}", a, b);
                    // }
                }
            }
        }
    }
}
