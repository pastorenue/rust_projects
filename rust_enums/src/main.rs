
#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddressKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Create,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

trait IntOrString {
    fn stringify(&self) -> String;
}

impl IntOrString for i32 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl IntOrString for &str {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Message {
    fn call(&self) {
        println!("Called with: {:?}", &self);
    }

    fn make_move(&self, x: i32, y: i32) -> Message {
        Message::Move {x, y}
    }

    fn from_value(&self, value: impl IntOrString) -> String {
        value.stringify()
    }
}

fn main() {
    let four: IpAddressKind = IpAddressKind::V4;
    let six: IpAddressKind = IpAddressKind::V6;
    let x = 5;
    let some_string = Some("hello");
    let absent_number: Option<i32> = Some(34);
    let sum: i32 = x + absent_number.unwrap_or(0);

    println!("{:?}", sum);
    println!("{:?}", some_string);
    println!("{:?}", absent_number);
    println!("{:?}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    route(IpAddressKind::V4);

    let localhost: IpAddr = IpAddr {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("{:?}", localhost);
    println!("{:?}", Message::Create.from_value(45));
    println!("{:?}", Message::Create.from_value("Hello"));
    println!("{:?}", Message::Create.make_move(4, 5));
    dbg!(route(ip_kind: IpAddressKind::V4));

}

fn route(ip_kind: IpAddressKind) {
    println!("{:?}", ip_kind);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
