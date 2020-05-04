use crate::{client, link_handler};
use crate::accounts::links;
use crate::client::ApiKeyType;
use std::collections::HashMap;
use crate::link_handler::check_options;
use crate::client::ApiKeyType::Link;

async fn cancel_order(order_id: String) -> String {
    let lnk = link_handler::set_param(
        &link_handler::set_link_api(links::CANCEL_ORDER.to_string()),
        &String::from("orderId"),
        &order_id,
    );
    client::text(client::retrieve().delete(&lnk), ApiKeyType::Link).await
}

async fn get_order(order_id: String) -> String {
    let lnk = link_handler::set_param(
        &link_handler::set_link_api(links::CANCEL_ORDER.to_string()),
        &String::from("orderId"),
        &order_id,
    );
    client::text(client::retrieve().get(&lnk), ApiKeyType::Link).await
}

async fn get_order_by_path(options: HashMap<String, String>) -> String {
    if check_options(&links::ORDER_BY_OPTIONS, &options) {
        let lnk = link_handler::set_link_api(links::GET_ORDERS_BY_PATH.to_string());
        client::text(client::retrieve().get(&lnk).query(&options),
                     ApiKeyType::Link).await
    } else {
        panic!("Required options not met.");
    }
}

async fn get_order_by_query(options: HashMap<String, String>) -> String {
    if check_options(&links::ORDER_BY_OPTIONS, &options) {
        client::text(
            client::retrieve().get(links::GET_ORDERS_BY_QUERY).query(&options),
            ApiKeyType::AccountId,
        ).await
    } else {
        panic!("Required options not met.");
    }
}

async fn place_order() -> String{
    panic!("Need to post json data, figure it out later");
    let lnk = link_handler::set_link_api(links::PLACE_ORDER.to_string());
    client::text(
        client::retrieve().post(&lnk),
        ApiKeyType::Link
    ).await
}

async fn replace_order(order_id: String) -> String {
    panic!("Need to put json data, figure it out later");
    let mut lnk = link_handler::set_link_api(links::REPLACE_ORDER.to_string());
    lnk = link_handler::set_param(&lnk, &String::from("orderId"), &order_id);
    client::text(
        client::retrieve().put(&lnk),
        ApiKeyType::Link
    ).await
}