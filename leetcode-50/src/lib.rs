pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        match n {
            0 if x != 0.0 => 1.0,
            _ if x == 1.0 => 1.0,
            _ if x == -1.0 => {
                if n % 2 == 0 {
                    1.0
                } else {
                    -1.0
                }
            }
            1 => x,
            i32::MIN => 0.0,
            negative if n < 0 => Solution::my_pow(1.0 / x, -negative),
            even if n % 2 == 0 => {
                let abs_x = x.abs();
                if abs_x < f64::EPSILON / abs_x {
                    0.0
                } else {
                    Solution::my_pow(abs_x * abs_x, even / 2)
                }
            }
            odd if n % 2 != 0 => {
                let abs_x = x.abs();
                if abs_x < f64::EPSILON / abs_x {
                    0.0
                } else {
                    x * Solution::my_pow(abs_x * abs_x, odd / 2)
                }
            }
            _ => f64::NAN,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[macro_export]
    macro_rules! pow_test {
        ($name: ident, ($base: literal, $power: literal) => $output: literal) => {
            #[test]
            fn $name() {
                let result = Solution::my_pow($base, $power);
                let abs_difference = (result - $output).abs();
                dbg!(abs_difference);
                assert!(abs_difference <= 1e-14);
            }
        };
    }

    pow_test!(test_simple_pow, (2.00000, 10) => 1024.00000);
    pow_test!(test_second_pow, (2.10000, 3) => 9.26100);
    pow_test!(test_negative_pow, (2.00000, -2) => 0.25000);
    pow_test!(test_too_small, (2.00, -200000000) => 0.0);
    pow_test!(test_negative_base, (-2.00000, 2) => 4.00000);
    pow_test!(test_one_any_pow, (1.0000, -2147483648) => 1.0000);
}
