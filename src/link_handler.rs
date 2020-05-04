use crate::key;
use std::collections::HashMap;
use std::borrow::Borrow;

pub(crate) fn set_link_api(link: String) -> String {
    str::replace(&link, "{apikey}", key::API_KEY)
}

pub(crate) fn set_param(lnk: &String, key: &String, value: &String) -> String {
    let k = format!("{{ {} }}", key);
    str::replace(lnk, &k, &value)
}

pub(crate) fn set_link_params(link: String, options: HashMap<String, String>) -> String {
    let mut lnk = link;
    for (key, value) in &options {
        lnk = set_param(&lnk, key, value);
    }
    lnk
}

pub(crate) fn check_options(keys: &[&str], options: &HashMap<String, String>) -> bool {
    for k in keys {
        if !options.contains_key(*k) {
            return false;
        }
    }
    true
}