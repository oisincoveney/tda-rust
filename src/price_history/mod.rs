mod links;

mod price_history {
    use crate::client::client::Client;
    use std::collections::HashMap;

    struct PriceHistory;

    impl PriceHistory {
        async fn get_price_history(symbol: &str, option_vec: Vec<(String, String)>) {
            let slice: &str = links::GET_PRICE_HISTORY.to_string();
            let lnk = format!(slice, symbol = symbol);

            let c = Client::auth(
                Client::retrieve().get(lnk)
            );

            let response = c.query(option_vec)
                .send()
                .await
                .unwrap();

            println!("{:?}", response);
            let text = response.text().await;
            println!("{:?}", text);
        }
    }
}