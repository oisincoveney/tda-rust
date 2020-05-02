pub mod quotes {
    use crate::client::client::Client;
    use crate::key::key::KEY;
    use crate::key::key::AUTH;

    pub(crate) struct Quote {
        link: String
    }

    impl Quote {
        pub async fn get_quote(symbol: &str) {
            let lnk = format!("https://api.tdameritrade.com/v1/marketdata/{symbol}/quotes", symbol = symbol);

            println!("{:?}", lnk);

            let response =
                Client::auth(
                    Client::retrieve()
                        .get(&lnk)
                )
                    .send()
                    .await
                    .unwrap();
            println!("{:?}", response);
            let text = response.text().await;
            println!("{:?}", text);
        }
    }
}