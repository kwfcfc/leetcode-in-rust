use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        fn rob_helper(nums_slice: &[i32]) -> i32 {
            let length = nums_slice.len();

            if length == 1 {
                return nums_slice[0];
            }

            let mut previous = nums_slice[0];
            let mut neighbor = if nums_slice[1] > previous {
                nums_slice[1]
            } else {
                previous
            };

            for &num in nums_slice.iter().skip(2) {
                let temp = neighbor;
                if neighbor  < previous + num {
                    neighbor = previous + num;
                }
                previous = temp;
            }

            neighbor
        }

        let length = nums.len();

        if length == 1 {
            return nums[0];
        }

        max(
            rob_helper(&nums[0..length - 1]),
            rob_helper(&nums[1..length]),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! rob_test {
        ($name: ident, $input: expr => $output: literal) => {
            #[test]
            fn $name() {
                let result = Solution::rob($input);
                assert_eq!(result, $output);
            }
        };
    }

    rob_test!(test_3_houses, vec![2,3,2] => 3);
    rob_test!(test_4_houses, vec![1,2,3,1] => 4);
    rob_test!(test_123, vec![1,2,3] => 3);
}
