pub struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 || n == 1 || n < 0 || n > 30 {
            return n;
        }

        let mut a = 0;
        let mut b = 1;
        for _i in 2..=n {
            (a, b) = (b, a + b);
        }
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! fib_test {
        ($name: ident, $input: literal => $output: literal) => {
            #[test]
            fn $name() {
                let result = Solution::fib($input);
                assert_eq!(result, $output);
            }
        };
    }

    fib_test!(test_2, 2 => 1);
    fib_test!(test_3, 3 => 2);
    fib_test!(test_4, 4 => 3);
}
