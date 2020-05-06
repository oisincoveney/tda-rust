mod links;

use crate::client;
use crate::client::ApiKeyType;
use std::collections::HashMap;

pub(crate) async fn get_option_chain(options: &HashMap<&str, &str>) -> String {
    client::text(
        client::retrieve()
            .get(links::GET_OPTION_CHAIN)
            .query(&options),
        ApiKeyType::Query
    ).await
}