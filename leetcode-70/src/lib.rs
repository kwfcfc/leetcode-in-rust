pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        } else if n == 2 {
            return 2;
        }

        let mut now = 2;
        let mut previous = 1;

        for _ in 2..n {
            let temp = now + previous;
            previous = now;
            now = temp;
        }
        now
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! climb_stairs_test {
        ($name: ident, $input: literal => $output: literal) => {
            #[test]
            fn $name() {
                let result = Solution::climb_stairs($input);
                assert_eq!(result, $output);
            }
        };
    }

    climb_stairs_test!(test_simple, 2 => 2);
    climb_stairs_test!(test_three, 3 => 3);
}
