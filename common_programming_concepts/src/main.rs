fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);
    println!("{}", common_programming_concepts::gcd(45, 15));
}

mod common_programming_concepts {
    pub fn gcd(mut m: u64, mut n: u64) -> u64 {
        assert!(m != 0 && n != 0);
        while m != 0 {
            if m < n {
                let t = m;
                m = n;
                n = t;
            }
            m = m % n;
        }
        n
    }

    pub fn _return_tuple() -> (&'static str, i32) {
        let tup: (&str, i32) = ("hello", 5);
        (tup.0, tup.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::common_programming_concepts;

    #[test]
    fn test_gcd() {
        assert_eq!(common_programming_concepts::gcd(14,5), 1);
        assert_eq!(common_programming_concepts::gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
    }

    #[test]
    fn test_return_tuple() {
        assert_eq!(common_programming_concepts::_return_tuple(), ("hello", 5));
    }
}