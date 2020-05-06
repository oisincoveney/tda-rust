mod links;

pub mod quotes {
    use crate::client;
    use crate::quotes::links;
    use crate::client::ApiKeyType;

    pub async fn get_quote(symbol: &str) -> String {
        let lnk = str::replace(links::GET_SINGLE_QUOTE, "{symbol}", symbol);

        client::text(client::retrieve().get(&lnk), ApiKeyType::Query).await
    }

    pub async fn get_quotes(symbols: Vec<String>) -> String {
        let sym_str: String = symbols.join(",");

        client::text(client::retrieve()
                         .get(links::GET_QUOTE)
                         .query(&["symbols", &sym_str]), ApiKeyType::Query,
        ).await
    }
}