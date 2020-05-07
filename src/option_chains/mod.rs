mod links;
extern crate serde;



use crate::client;
use crate::client::ApiKeyType;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

pub(crate) async fn get_option_chain(options: &Vec<(&str, &str)>) -> OptionChain {
    let data: Result<OptionChain, reqwest::Error> = client::json(
        client::retrieve()
            .get(links::GET_OPTION_CHAIN)
            .query(&options),
        ApiKeyType::Query
    ).await;
    data.unwrap()
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionChain {
    pub symbol: String,
    pub status: String,
    pub underlying: Underlying,
    pub strategy: String,
    pub interval: f64,
    #[serde(rename = "isDelayed")]
    pub is_delayed: bool,
    #[serde(rename = "isIndex")]
    pub is_index: bool,
    #[serde(rename = "interestRate")]
    pub interest_rate: f64,
    #[serde(rename = "underlyingPrice")]
    pub underlying_price: f64,
    pub volatility: f64,
    #[serde(rename = "daysToExpiration")]
    pub days_to_expiration: f64,
    #[serde(rename = "numberOfContracts")]
    pub number_of_contracts: i64,
    #[serde(rename = "callExpDateMap")] //HashMap<String, serde_json::Value>
    pub call_exp_date_map: HashMap<String, HashMap<String, Vec<DataValues>>>,// CallExpDateMap,
    #[serde(rename = "putExpDateMap")]
    pub put_exp_date_map: HashMap<String, serde_json::Value>, // PutExpDateMap,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Underlying {
    pub symbol: String,
    pub description: String,
    pub change: f64,
    #[serde(rename = "percentChange")]
    pub percent_change: f64,
    pub close: f64,
    #[serde(rename = "quoteTime")]
    pub quote_time: i64,
    #[serde(rename = "tradeTime")]
    pub trade_time: i64,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub mark: f64,
    #[serde(rename = "markChange")]
    pub mark_change: f64,
    #[serde(rename = "markPercentChange")]
    pub mark_percent_change: f64,
    #[serde(rename = "bidSize")]
    pub bid_size: i64,
    #[serde(rename = "askSize")]
    pub ask_size: i64,
    #[serde(rename = "highPrice")]
    pub high_price: f64,
    #[serde(rename = "lowPrice")]
    pub low_price: f64,
    #[serde(rename = "openPrice")]
    pub open_price: f64,
    #[serde(rename = "totalVolume")]
    pub total_volume: i64,
    #[serde(rename = "exchangeName")]
    pub exchange_name: String,
    #[serde(rename = "fiftyTwoWeekHigh")]
    pub fifty_two_week_high: f64,
    #[serde(rename = "fiftyTwoWeekLow")]
    pub fifty_two_week_low: f64,
    pub delayed: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataValues {
    pub put_call: String,
    pub symbol: String,
    pub description: String,
    pub exchange_name: String,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub mark: f64,
    pub bid_size: f64,
    pub ask_size: f64,
    pub bid_ask_size: String,
    pub last_size: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub open_price: f64,
    pub close_price: f64,
    pub total_volume: f64,
    pub trade_date: ::serde_json::Value,
    pub trade_time_in_long: f64,
    pub quote_time_in_long: f64,
    pub net_change: f64,
    pub volatility: f64,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub rho: f64,
    pub open_interest: f64,
    pub time_value: f64,
    pub theoretical_option_value: f64,
    pub theoretical_volatility: f64,
    pub option_deliverables_list: ::serde_json::Value,
    pub strike_price: f64,
    pub expiration_date: f64,
    pub days_to_expiration: f64,
    pub expiration_type: String,
    pub last_trading_day: f64,
    pub multiplier: f64,
    pub settlement_type: String,
    pub deliverable_note: String,
    pub is_index_option: ::serde_json::Value,
    pub percent_change: f64,
    pub mark_change: f64,
    pub mark_percent_change: f64,
    pub in_the_money: bool,
    pub mini: bool,
    pub non_standard: bool,
}

// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct CallExpDateMap {
//     chain: HashMap<String, serde_json::Value>
// }
//
// #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct PutExpDateMap {
//     chain: HashMap<String, serde_json::Value>
// }