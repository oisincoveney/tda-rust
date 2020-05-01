#[macro_use]
extern crate lazy_static;

mod client;

use client::client::CLIENT;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        penis.get("https://www.google.com");
    }
}
