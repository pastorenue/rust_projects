#[allow(dead_code)]

mod utils;

use utils::api::APIClient;
use utils::helpers::StaticArg;

fn main() {
    let client = APIClient::new(StaticArg {
        marketplace: String::from("test"),
        api_key: String::from("test"),
    });
    println!("{:?}", client.get_orders(vec![1,2,3,4,5]));

}