use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    /*
    This is not possible because y is a reference to x.
    comparing a number and a reference to a number is not allowed
    because they are of different types.
    In this case, use the deref operator
     */
    // assert_eq!(5, y); // error
    assert_eq!(5, *y); // ok

    let _a = [1,2,3];
    let mut v:Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    println!("{:?}", v);

    {
        // Only visible within this scope
        let vec_2 = vec![1,2,3,4,5];
        println!("{:?}", vec_2);
    }

    let third = v[2];
    v.push(4);
    let t: i32 =v[3];
    println!("The third element is {}", third);
    println!("The third element is {}", t);

    match v.get(20) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &v {
        println!("{}", i+40); // This is a copy
    }

    for i in &mut v {
        *i += 50; // This mutates the vector and is not a copy
        println!("{}", i);
    }


    for i in "This is my name".split_whitespace() {
        println!("{}", i);
    }

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    for g in "Gutnerberg".graphemes(true) {
        println!("{}", g);
    }

    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3: String = s1.clone() + &s2;

    println!("{}", s3);
    println!("{}", s1);

    // Hashmap
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("blue"), 32);
    scores.entry(String::from("yellow")).or_insert(10);

    println!("{:?}", scores);

    let text: &str = "You can only be the best of what you are.";
    let mut map = HashMap::new();
    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    //     // You can do this in one line as well
    //     // *map.entry(word).or_insert(0) += 1;
    // }

    // For case insensitive
    let lower_case_text = text.to_lowercase();

    for word in lower_case_text.split_whitespace() {
        *map.entry(word).or_insert(0) += 1;
    }
    println!("{:?}", map);

}
