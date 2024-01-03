const NAME_SIZE: u8 = 23;

// Rust variables
fn defining_variables() {
    // rust infers the type of x
    let x = 13;
    println!("{}", x);

    // rust can also be explicit about the type
    let x: f64 = 3.14159;
    println!("{}", x);

    // rust can also declare and initialize later
    let x;
    x = 12;
    println!("{}", x);
}

fn changing_variables() {
    // mutable variables allows to be written to and read from at compile time
    let mut x = 43;
    println!("{}", x);

    x = 13; // we can modify the variable because its `mut` value
    println!("{}", x);
}

// booleans - bool for representing true/false
// unsigned integers - u8 u16 u32 u64 u128 for representing nonnegative whole numbers
// signed integers - i8 i16 i32 i64 i128 for representing whole numbers
// pointer sized integers - usize isize for representing indexes and sizes of things in memory
// floating point - f32 f64
// tuple - (value, :value, ...) for passing fixed sequences of values on the stack
// arrays - [value, value, ...] a collection of similar elements with fixed length known at compile time
// slices - a collection of similar elements with length known at runtime
// str(string slice) - text with a length known at runtime
fn rust_types() {
    let x = 12; // by default this is i32
    let a = 12u8;
    let b = 4.3; // by default this is f64
    let c = 4.3f32;
    let bv = true; 
    let t = (13, false);
    let sentence = "hello world";
    println!("{} {} {} {} {} {} {} {} {}", x, a, b, c, t.1, t.0, bv, sentence, NAME_SIZE);

    // To convert from type to another type, we use the `as` keyword
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;

    println!("{:3}", c as f32);
}

fn declaring_arrays() {
    // signature is [T;N] ==> [T;N] ==> [element_type; fixed_length];
    let numbers: [i8;4] = [12,5,4,6];
    println!("{:?}", numbers);
    println!("{}", numbers[0]);
}

fn math_operation(x: i32, y: i32, operator: &str) -> i32 {
    match operator {
        "+" => x + y,
        "*" => x * y,
        "/" => x / y,
        "-" => x - y,
        _ => 0
    }
}

fn first_and_last() -> (i32, i32) {
    let tuple: (i32, i32) = (0, 0);
    return (tuple.0, tuple.1);
}

fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}
fn main() {
    println!("Hello, world!");
    defining_variables(); // Defining types in rust
    changing_variables(); // changing variables in rust
    rust_types(); // Rust types
    declaring_arrays(); // Declaring arrays in rust
    println!("{}", math_operation(23,45, "-")); // Declaring a function in rust
    println!("{:?}", first_and_last()); // Returning a tuplein a function in rust

    let result = swap(4,5);
    println!("{} {}", result.0, result.1);
}
