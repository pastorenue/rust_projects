pub struct StaticArg {
    pub marketplace: String,
    pub api_key: String,
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Order {
    sku: String,
    quantity: i32,
    price: f64,
    title: String,
    condition: String,
}

impl Order {
    pub fn new(sku: String, quantity: i32, price: f64, title: String, condition: String) -> Order {
        Order {
            sku,
            quantity,
            price,
            title,
            condition
        }
   } 
}