extern crate tokio;

mod key;
mod client;
mod quotes;

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
}
