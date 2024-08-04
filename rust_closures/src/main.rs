// Closures are anonymous functions that can be passed around as data.
use std::{cell::Cell, vec};

#[derive(Debug)]
#[allow(dead_code)]
struct Person<'p> {
    first_name: String,
    last_name: Cell<&'p str>,
    age: Cell<i32>, // Cell is a thread-safe interior mutability container
}
fn main() {
    // test_closures();
    // let inventory = Inventory {
    //     shirts: vec![ShirtColor::Red, ShirtColor::Red, ShirtColor::Blue],
    // };
    // shirt_giveaway(&inventory)
    let mut cacher = Cacher::new(|x| x + 1);
    println!("The result is {}", cacher.compute(None));
    println!("The values in the cache are {:?}", cacher.values());
    unsafe {
        paly_with_closures();
    }
}

#[allow(dead_code)]
fn test_closures() {
    let x = 5;
    let add = |x: i32| println!("Hello, world! {}", x); // The closure does not mutate its environment
    add(2);

    let sum = |x: i32, y: i32| x + y;
    println!("The result is {}", sum(2,3));

    let mut_x = |z: i32| x + z;
    println!("The result is {}", mut_x(2));

    let p1: Person<'_> = new_person("John", "Doe");

    println!("The person is {:?}", p1);
}

#[allow(dead_code)]
fn new_person<'p>(first_name: &str, last_name: &'p str) -> Person<'p> {
    let p1: Person<'p> = Person {
        first_name: String::from(first_name),
        last_name: Cell::from(last_name),
        age: Cell::from(32),
    };
    return p1;
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

#[allow(dead_code)]
struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    #[allow(dead_code)]
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}


#[allow(dead_code)]
fn shirt_giveaway(inventory: &Inventory) -> () {
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway = inventory.giveaway(user_pref1);
    println!(
        "The user with pref {:?} get {:?}",
        user_pref1, giveaway
    );

    let user_pref2 = None;
    let giveaway = inventory.giveaway(user_pref2);
    println!(
        "The user with pref {:?} get {:?}",
        user_pref2, giveaway
    )
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    values: Vec<Option<u32>>
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: vec![],
        }
    }

    fn compute(&mut self, v: Option<u32>) -> u32 {
        let result = (self.calculation)(v.unwrap_or_default());
        self.values.push(v);
        result
    }

    fn values(&self) -> &Vec<Option<u32>> {
        &self.values
    }
}

unsafe fn paly_with_closures() {
    let bytes: [u8; 7] = [1, 2, 3, 4, 5, 6, 7]; // 7 bytes
    let (prefix, shorts, suffix) = bytes.align_to::<u16>(); // align the bytes to u16
    let slice = &bytes[..]; // slice of 7 bytes
    let closure = |x: &[u8]| -> usize { // closure that takes a slice of u8 and returns a usize
        x.iter().map(|&x| x as usize).sum() // sum of the bytes in the slice
    };
    let result = closure(slice); // call the closure with the slice
    println!("{}", result); // 28
    println!("prefix: {:?}, shorts: {:?}, suffix: {:?}", prefix, shorts, suffix);
}