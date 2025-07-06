pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 0.0 {
            if n > 0 {
                return 0.0;
            } else {
                return f64::NAN;
            }
        } else if x == 1.0 {
            return 1.0;
        } else if x == -1.0 {
            match n % 2 == 0 {
                true => return 1.0,
                false => return -1.0,
            }
        }
        let mut result = 1.0;
        let mut pow = n;
        let mut base = x;

        if pow < 0 {
            if pow > i32::MIN {
                pow = -pow;
                base = 1.0 / base
            } else {
                return 0.0;
            }
        }
        while pow != 0 {
            if (pow & 1).is_positive() {
                result *= base;
            }
            base *= base;
            pow >>= 1;
        }
        result
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
