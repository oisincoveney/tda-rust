extern crate serde;

mod links;

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

        ph_data.unwrap()
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PriceHistoryData {
        pub candles: Candles,
        pub empty: Empty,
        pub symbol: Symbol,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Candles {
        #[serde(rename = "type")]
        pub type_field: String,
        pub description: String,
        pub items: Items,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Items {
        #[serde(rename = "type")]
        pub type_field: String,
        pub properties: Properties,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Properties {
        pub close: Close,
        pub datetime: Datetime,
        pub high: High,
        pub low: Low,
        pub open: Open,
        pub volume: Volume,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Close {
        #[serde(rename = "type")]
        pub type_field: String,
        pub format: String,
        pub description: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Datetime {
        #[serde(rename = "type")]
        pub type_field: String,
        pub format: String,
        pub description: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct High {
        #[serde(rename = "type")]
        pub type_field: String,
        pub format: String,
        pub description: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Low {
        #[serde(rename = "type")]
        pub type_field: String,
        pub format: String,
        pub description: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Open {
        #[serde(rename = "type")]
        pub type_field: String,
        pub format: String,
        pub description: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Volume {
        #[serde(rename = "type")]
        pub type_field: String,
        pub format: String,
        pub description: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Empty {
        #[serde(rename = "type")]
        pub type_field: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Symbol {
        #[serde(rename = "type")]
        pub type_field: String,
        pub description: String,
    }
}