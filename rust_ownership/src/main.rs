fn main() {
    let s: String = gives_ownership();
    let s2 = String::from("hello");
    let mut s3 = takes_and_gives_ownership(s2);
    let len = calculate_length_using_ref(&s3);
    change_str(&mut s3);
    let x = 4;

    println!("{:?}", s);

    takes_ownership(s);
    makes_copy(x);
    println!("The length of the '{}' is {}", s3, len);

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(other: i32) {
    println!("{}", other);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_ownership(some_string: String) -> String {
    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_using_ref(s: &String) ->  usize {
    s.len()
}

fn change_str(s: &mut String) { 
    // This won't allow us modify the string since its borrowed, 
    // unless we use `&mut String` in the argument.
    s.push_str(", world");
}