#![feature(test)]

extern crate test;

/// This function adds two to the argument
///
/// # Examples
///
/// ```
/// use adder::add_two;
///
/// assert_eq!(302, add_two(300));
/// ```
pub fn add_two(i: i32) -> i32 {
    i + 2
}

#[cfg(test)]

mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn add_two_adds_two() {
        assert_eq!(102, add_two(100));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }
}
