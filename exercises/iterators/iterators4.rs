// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.


pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return :: done
    // Try not to use:
    // - imperative style loops (for, while) :: done
    // - additional variables :: done (ish, snuck in memo + n --but in a different context)
    // For an extra challenge, don't use:
    // - recursion :: done
    // Execute `rustlings hint iterators4` for hints.

    match num {
        0 | 1 => 1,
        2..= u64::MAX => (1..= num).into_iter().reduce(|memo, n| memo * n).unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
