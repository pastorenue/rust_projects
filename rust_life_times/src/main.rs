// The following are possible lifetime positions:
// - `'a`
// - `&'a T`
// - `&'a mut T`
// - `T`
// - `T: 'a`
// - `where T: 'a`


fn main() {
    // first_lifetime();
    // second_lifetime(&x, &y);
    // Any input which is borrowed must outlive the borrower. 
    // In other words, the lifetime of `x` and `y` must 
    // be longer than that of `second_lifetime`.

    // failed_borrow();
    // `failed_borrow` contains no references to force `'a` to be 
    // longer than the lifetime of the function, but `'a` is longer.
    // Because the lifetime is never constrained, it defaults to `'static`.
    // let string1: String = String::from("abcd");
    
    // let result: &str;
    // {
    //     let string2: String = "xyz".to_string();
    //     result = return_first_parameter(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
    let result = lifetime_returns_literal();
    println!("{}", result);

}


#[allow(dead_code)]
fn first_lifetime() {
    let i = 3; // -+ i goes into scope and lifetime starts
    {
        let borrow1 = &i; // -+ borrow1 goes into scope and lifetime starts
        println!("borrow1: {}", borrow1);
    } // -+ borrow1 goes out of scope and lifetime ends

    {
        let borrow2 = &i; // -+ borrow2 goes into scope and lifetime starts
        println!("borrow2: {}", borrow2);
    } // -+ borrow2 goes out of scope and lifetime ends
} // -+ i goes out of scope and lifetime ends


// lifetimes with annotations
// `second_lifetime` takes two references to `i32` which have different
// lifetimes `'a` and `'b`. These two lifetimes must both be at
// least as long as the function `print_refs`.
#[allow(dead_code)]
fn second_lifetime<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

// A function which takes no arguments, but has a lifetime parameter `'a`.
// fn failed_borrow<'a>() {
//     let _x = 12;

//     // ERROR: `_x` does not live long enough
//     let _y: &'a i32 = &_x;
//     // Attempting to use the lifetime `'a` as an explicit type annotation 
//     // inside the function will fail because the lifetime of `&_x` is shorter
//     // than that of `_y`. A short lifetime cannot be coerced into a longer one.
// }

#[allow(dead_code)]
fn another_example() {
    {
        let x: i32 = 5;
        let r: &i32 = &x;
    
        println!("{}", r);
    }
    } // -+ `x` goes out of scope

#[allow(dead_code)]
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str { // -+ returns the smallest lifetime of x and y
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// This function returns a reference to a static string which is stored in the program's memory.
// and lives as long as the program.
fn lifetime_returns_literal() -> &'static str {
    return "hello, world";
}


#[allow(dead_code)]
fn return_first_parameter<'a>(a: &'a str , _b: &str) -> &'a str{
    a
}


// lifetime elision rulse
// - Each parameter that is reference gets its own lifetime parameter.
// -1. If there is exactly one input lifetime parameter, that lifetime is referred to as the `'a` lifetime.
// -2. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is not included.
#[allow(dead_code)]
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}