extern crate reqwest;

/**
 *
 * Client for TD Ameritrade rest calls
 *
 * */
pub mod client {
    type RClient = reqwest::Client;

    lazy_static! {
        static ref GO: RClient = RClient::new();
    }

    pub struct Client {
        access_code: String,
        refresh_code: String,
        key: String,
    }

    impl Client {
        fn get() {}
        fn post() {}
        fn put() {}
        fn delete() {}
    }
}
