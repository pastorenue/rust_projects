fn main() {
    println!("Hello, world!");
    another();
    another_with_params(12);
    function_wih_more_params(4, 'G');
    let amount = formatted_currency(450, "GBP");
    println!("Amount: {:?}", amount);

    // A simple statement-expression
    let y = {
        let x = 45;
        x + 1
    };
    println!("y: {:?}", y);
    let fv = (plus_two(45), five());
    println!("fv: {:?}", fv);
}

fn another() -> () {
    println!("Another World!");
}

fn another_with_params(x: i32) {
    println!("The value of x is: {}", x);
}

fn function_wih_more_params(x: i32, y: char) {
    println!("The values are: x-{} and y-{}", x, y);
}

fn formatted_currency(amount: i32, currency_code: &str) -> String {
    format!("{}{}", amount, currency_code)
}

fn five() -> i32 {
    5
}

fn plus_two(x: i32) -> i32 {
    x + 2 // Adding a semicolon to the end will change this to a statement
}