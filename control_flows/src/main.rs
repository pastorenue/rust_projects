fn main() {
    // test_if();
    // test_while();
    // test_loop();
    test_for();
}

#[allow(dead_code)]
fn test_if() {
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

#[allow(dead_code)]
fn test_while() {
    let age_to_drive = 16u8;

    let mut current_age = 0u8;
    while current_age < age_to_drive {
        println!("Waiting ...");
        current_age += 1;

        if current_age == 10 {
            println!("You are waiting for too long!");
            break;
        }
    }
}

#[allow(dead_code)]
fn test_loop() {
    let mut x = 1;

    loop {
        println!("x = {}", x);
        if x > 5 {
            break;
        }
        x += 1;
    }
}

fn test_for() {
    let ages = [18, 24, 13, 30, 17, 15, 60];
    let age_to_drive = 16u8;

    for age in ages {
        println!("Age -> {}:", age);
        if age > age_to_drive {
            println!("Age {} is eligible to drive", age);
        } else {
            println!("Age {} is not eligible to drive", age);
        }
    }
}