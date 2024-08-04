pub mod garden;

fn main() {
    phrases::greeting::hello();
    parent::greet();
    parent::child::hello();
}

mod phrases {
    pub mod greeting {
        pub fn hello() { // function has to be public to be accessed from outside the module.
            println!("Hello, world!");
        }
    }
}

// Private functions can be called from the same module or child module
mod parent {
    pub fn greet() {
        hello();
    }

    fn hello() {
        println!("Hello, world!");
    }

    pub mod child {
        pub fn hello() {
            super::hello();
        }
    }
}