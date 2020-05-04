extern crate reqwest;

/**
 * Client for TD Ameritrade rest calls
 *
 * */
// use key::AUTH;
// use key::REFRESH;
//
// use std::borrow::Borrow;

use crate::key::API_KEY;
use crate::key::AUTH;
use serde::de::DeserializeOwned;

#[derive(PartialEq, Eq)]
pub enum ApiKeyType {
    Query,
    AccountId,
    Link
}

pub fn retrieve() -> reqwest::Client {
    reqwest::Client::new()
}

pub fn auth(rb: reqwest::RequestBuilder, key_type: ApiKeyType) -> reqwest::RequestBuilder {
    let r = rb.header("Bearer", AUTH);
    if key_type == ApiKeyType::Query {
        r.query(&[("apikey", API_KEY)])
    } else if key_type == ApiKeyType::AccountId {
        r.query(&["accountId", API_KEY])
    } else {
        r
    }
}

pub async fn text(rb: reqwest::RequestBuilder, key_type: ApiKeyType) -> String {
    match auth(rb.header("Bearer", AUTH), key_type)
        .send()
        .await
        .unwrap()
        .text()
        .await {
        Ok(val) => val,
        Err(e) => e.to_string()
    }
}

pub async fn json<T: DeserializeOwned>(rb: reqwest::RequestBuilder, key_type: ApiKeyType) -> Result<T, reqwest::Error> {
    let data: Result<T, reqwest::Error> = auth(rb.header("Bearer", AUTH), key_type)
        .send()
        .await
        .unwrap()
        .json::<T>()
        .await;

    data
}
// }
