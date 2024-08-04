#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

trait RectangleInterface {
    /*
    Add a docstring to the function
    */
    fn new(width: u32, height: u32) -> Rectangle;

    fn area(&self) -> u32;

    fn can_hold(&self, other: &Rectangle) -> bool;

    fn square(size: u32) -> Rectangle;
}

impl RectangleInterface for Rectangle {
    /*
    Add a docstring to the function
    */
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0);
    let rect1 = Rectangle::new(30,50);
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle::square(30);

    println!("The rectangle can hold rect2 {:#?}", rect3);

    let user1 = User {
        email: String::from("9uGq0@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = build_user(String::from("9uGq0@example.com"), String::from("someusername123"));
    let user3 = User {
        email: String::from("9uGq0@example.com"),
        username: String::from("someusername123"),
        ..user1
    };
    println!("user3: {:?}", user3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
