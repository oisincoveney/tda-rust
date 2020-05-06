mod links;

use crate::client;
use crate::client::ApiKeyType;
use std::collections::HashMap;

pub(crate) async fn get_option_chain(options: &HashMap<&str, &str>) -> String {
    /*
    GET /v1/marketdata/chains?apikey=VCSOJTWA76YNGGQAMSVVCVADRXWNWXU8&symbol=SPY&contractType=ALL&strikeCount=30&includeQuotes=TRUE&strategy=SINGLE&range=ALL&fromDate=2020-03-01&toDate=2020-06-05&expMonth=ALL&optionType=ALL HTTP/1.1
     */
    client::text(
        client::retrieve()
            .get(links::GET_OPTION_CHAIN)
            .query(&options),
        ApiKeyType::Query
    ).await
}