extern crate tokio;
mod key;
mod client;
mod quotes;
mod price_history;

use crate::quotes::quotes::Quote;

#[cfg(test)]
mod tests {
    use crate::quotes::quotes::Quote;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[tokio::test]
    async fn titties () {
        Quote::get_quote("SPY").await;
    }

    #[tokio::test]
    async fn titties2 () {
        let options = vec![
            ("periodType", "day"),
            ("period", 10),
            ("frequencyType", minute),
            ("frequency", 10)
        ];
    }
}
