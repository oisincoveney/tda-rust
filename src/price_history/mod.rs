extern crate serde;


mod links;
pub mod ml_testing;

pub(crate) mod price_history {
    use crate::client;
    use crate::price_history::links::GET_PRICE_HISTORY;
    use crate::client::ApiKeyType;
    use serde::{Serialize, Deserialize};

    pub(crate) async fn get_price_history(symbol: &str, option_vec: &Vec<(&str, &str)>) -> String {
        let lnk = str::replace(GET_PRICE_HISTORY, "{symbol}", symbol);

        client::text(
            client::retrieve().get(&lnk).query(&option_vec), ApiKeyType::Query
        ).await
    }

    pub(crate) async fn get_price_history_json(symbol: &str, option_vec: &Vec<(&str, &str)>) -> PriceHistoryData {
        let lnk = str::replace(GET_PRICE_HISTORY, "{symbol}", symbol);

        let ph_data= client::json(
            client::retrieve().get(&lnk).query(&option_vec), ApiKeyType::Query
        ).await;

        match ph_data {
            Ok(x) => x,
            Err(_) => unimplemented!(),
        }
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct PriceHistoryData {
        pub candles: Vec<Candle>,
        pub symbol: String,
        pub empty: bool,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Candle {
        pub open: f64,
        pub high: f64,
        pub low: f64,
        pub close: f64,
        pub volume: i64,
        pub datetime: i64,
    }

}