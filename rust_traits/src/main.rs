#![allow(dead_code)]
mod math;

use math::shapes::{Square, Rectangle};
use math::interfaces::{PolygonalMath, VolumetricMath};
use math::solids::Solid;

fn main() {
    tryin_solid_traits();
}

fn tryin_shape_traits() {
    let rectangle = Rectangle {
        width: 10,
        height: 10,
    };
    let square = Square {
        side: 10
    };
    println!("{}", rectangle.as_str());
    println!("{}", square.as_str());
    println!("The area of the rectangle is {}", rectangle.area());
    println!("The perimeter of the rectangle is {}", rectangle.perimeter());
    println!("The area of the square is {}", square.area());
    println!("The perimeter of the square is {}", square.perimeter());
}

fn tryin_solid_traits() {
    let cube = Solid::Cube { side: 10 };
    let cuboid = Solid::Cuboid { side: 10 };
    let cylinder = Solid::Cylinder { radius: 10, height: 10 };
    println!("The volume of the cube is {}", cube.volume());
    println!("The volume of the cuboid is {}", cuboid.volume());
    println!("The volume of the cylinder is {}", cylinder.volume());
}

struct Tweet {
    author: String,
    reply: bool,
    retweet: bool,
    content: String,
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for Tweet {
    fn author_name(&self) -> String {
        format!("{}", self.author)
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, self.author)
    }

    fn author_name(&self) -> String {
        format!("{}", self.author)
    }
}

trait Summary {
    fn author_name(&self) -> String;

    fn summarize(&self) -> String {
        format!("This is default for: {}", self.author_name())
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Or trait bound
fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Using where...
fn notify3<T>(item: T) -> String
    where T: Summary {
    format!("Breaking news! {}", item.summarize())
}

fn return_values() {
    let tweet = Tweet {
        author: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Hello"),
        location: String::from("World"),
        author: String::from("Me"),
        content: String::from("Some content"),
    };

    println!("{}", tweet.summarize());
    println!("{}", article.summarize());
    println!("{}", notify3(tweet));
}