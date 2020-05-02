extern crate reqwest;
/**
 * Client for TD Ameritrade rest calls
 *
 * */
pub mod client {
    // use key::AUTH;
    // use key::REFRESH;
    //
    // use std::borrow::Borrow;

    use crate::key::key::KEY;
    use crate::key::key::AUTH;
    use core::future::Future;
    use reqwest::Error;
    use reqwest::Response;

    type RClient = reqwest::Client;
    type RBuilder = reqwest::RequestBuilder;

    pub struct Client;

    impl Client {
        pub fn retrieve() -> reqwest::Client {
            reqwest::Client::new()
        }

        pub fn auth(rb: RBuilder) -> reqwest::RequestBuilder {
            // rb.bearer_auth(AUTH)
            rb.header("Bearer", AUTH).query(&[("apikey", KEY)])
        }
    }
}
