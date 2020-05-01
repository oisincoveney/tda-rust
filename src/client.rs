extern crate reqwest;
extern crate state;

/**
 *
 * Client for TD Ameritrade rest calls
 *
 * */
pub mod client {

    use state::Storage;
    use std::sync::RwLock;

    type RClient = reqwest::Client;

    lazy_static! {
        pub static ref CLIENT: Storage<RwLock<reqwest::Client>> = Storage::new();
    }

    pub struct Client {
        access_code: String,
        refresh_code: String,
        key: String,
    }

    impl Client {
        fn create(
            access_code: String,
            refresh_code: String,
            key: String,
        ) -> &'static RwLock<reqwest::Client> {
            if CLIENT.try_get().is_none() {
                let c = Client {
                    access_code,
                    refresh_code,
                    key,
                };

                CLIENT.set(RwLock::new(reqwest::Client::new()));
            }

            CLIENT.get()
        }

        fn go() {
            if CLIENT.try_get().is_none() {
                panic!("No client found.");
            }
            CLIENT.get();
        }

        fn get(&self, link: &String) {
            Client::go();
            // .get(link.to_string())
            // .bearer_auth(self.access_code)
            // .query(&[("apikey", self.key)]);
        }

        // fn post(&self, &link) {}

        // fn put(&self, &link) {}

        // fn delete(&self, &link) {}
    }
}
