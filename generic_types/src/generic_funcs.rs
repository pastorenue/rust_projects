fn _largest<T>(_list: &[T]) -> () {
    // ----snip-----
}

#[cfg(target_env = "dev")]
fn _smallest<T>(_list: &[T]) -> () {
    // ----snip-----
}

#[cfg(all(target_os = "macos", target_family = "unix", target_vendor = "apple"))]
fn _largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


fn _largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn _get_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

#[cfg(feature="momox_enabled")]
pub fn _main() {
    println!("Hello, world!");
}