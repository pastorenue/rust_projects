use super::helpers::{StaticArg, Order};

#[derive(Debug)]
pub struct APIClient {
    pub marketplace: String,
    pub api_key: String,
}

impl APIClient {
    pub fn new(args: StaticArg) -> APIClient {
        APIClient {
            marketplace: args.marketplace,
            api_key: args.api_key
        }
    }

    pub fn get_orders(&self, order_ids: Vec<i32>) -> Vec<Order> {
        let condition_options = vec!["New", "Used", "Refurbished"];
        let mut order_list: Vec<Order> = vec![];
        for i in order_ids {
            let condition_id: usize = (i % 3) as usize;
            let price = (10 % i * 15) as f64;
            order_list.push(Order::new(format!("sku_{}", i), i, price, format!("title_{}", i), condition_options[condition_id].to_string()));
        }
        order_list
    }
}

pub fn orders(args: StaticArg) -> Vec<Order> {
    let client = APIClient::new(args);
    client.get_orders(vec![1,2,3,4,5,6])
}