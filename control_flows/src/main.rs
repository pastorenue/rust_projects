fn main() {
    let number = 8;
    if number < 5 {
        println!("Condition passed!");
    } else if number > 5{
        println!("Condition passed with {} greater than 5!", number);
    } else {
        println!("Condition failed!")
    }

    let condition = false;
    let value = if condition {6} else {0};
    println!("The value is {}", value);
    let x = fibonacci(6);
    println!("The value is {}", x);
}

fn loop_function() {
    loop {
        println!("Loop"); // Infinite loop
    }
}

fn loop_with_break() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}", result);
}

fn nested_loops() {
    let mut count = 0;
    'main: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'main;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    // let mut count = 3;
    // while count != 0 {
    //     println!("{count}");
    //     count -= 1;
    // }
    // println!("LIFTOFF!!!");
    let a = [10,20,30,40,50];
    // let mut index: usize = 0;

    // while index < 5 {
    //     println!("The value is: {}", a[index]);
    //     index += 1;
    // }
    // for element in a {
    //     println!("The value is: {}", element);
    // }
    for i in (0..4).rev() {
        println!("{}", i);
    }
    println!("LIFTOFF!!!");
}

fn fibonacci(x: i32) -> i32 {
    if x <= 1 {
        x
    } else {
        fibonacci(x-1) + fibonacci(x-2)
    }
}
