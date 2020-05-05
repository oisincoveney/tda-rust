/* ORDERS */
use std::collections::HashSet;

pub(crate) const CANCEL_ORDER: &str = "https://api.tdameritrade.com/v1/accounts/{apikey}/orders/{orderId}";
pub(crate) const GET_ORDER: &str = "https://api.tdameritrade.com/v1/accounts/{apikey}/orders/{orderId}";
pub(crate) const GET_ORDERS_BY_PATH: &str = "https://api.tdameritrade.com/v1/accounts/{apikey}/orders";
pub(crate) const GET_ORDERS_BY_QUERY: &str = "https://api.tdameritrade.com/v1/orders";
pub(crate) const ORDER_BY_OPTIONS: [&'static str; 4] = ["maxResults", "fromEnteredTime", "toEnteredTime", "status"];
pub(crate) const PLACE_ORDER: &str = "https://api.tdameritrade.com/v1/accounts/{apikey}/orders";
pub(crate) const REPLACE_ORDER: &str = "https://api.tdameritrade.com/v1/accounts/{apikey}/orders/{orderId}";

/* SAVED ORDERS */
pub(crate) const CREATE_SAVED_ORDER: &str = "https://api.tdameritrade.com/v1/accounts/{apikey}/savedorders";
pub(crate) const DELETE_SAVED_ORDER: &str = "https://api.tdameritrade.com/v1/accounts/{apikey}/savedorders/{savedOrderId}";
pub(crate) const GET_SAVED_ORDER: &str = "https://api.tdameritrade.com/v1/accounts/{apikey}/savedorders/{savedOrderId}";
pub(crate) const GET_SAVED_ORDERS_BY_PATH: &str = "https://api.tdameritrade.com/v1/accounts/{apikey}/savedorders";
pub(crate) const REPLACE_SAVED_ORDER: &str = "https://api.tdameritrade.com/v1/accounts/{apikey}/savedorders/{savedOrderId}";

/* ACCOUNTS */
pub(crate) const GET_ACCOUNT: &str = "https://api.tdameritrade.com/v1/accounts/{apikey}";
pub(crate) const GET_ACCOUNTS: &str = "https://api.tdameritrade.com/v1/accounts";