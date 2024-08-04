fn main() {
    test_input();
}

#[allow(dead_code)]
fn test_input() {
    let max_age = 16u8;
    println!("Enter your age: ");
    let my_input = &mut String::from("");
    std::io::stdin().read_line(my_input).unwrap();
    let age = my_input.replace("\n", "").parse::<u8>().unwrap();
    
    if age > max_age {
        println!("You are old enough to drive!");
    } else if age == max_age {
        println!("Congratulations Right on time!");
    } else {
        println!("You are not old enough to drive!");
    }
}
