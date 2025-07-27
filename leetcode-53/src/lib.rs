use std::{cmp::max, i32};

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut current = nums[0];
        let mut max_sum = current;

        for &num in nums.iter().skip(1) {
            current = max(num, current + num);
            max_sum = max(max_sum, current);
        }

        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! subarray_test {
        ($name: ident, $input: expr => $output: literal) => {
            #[test]
            fn $name() {
                let result = Solution::max_sub_array($input);
                assert_eq!(result, $output);
            }
        };
    }

    subarray_test!(test_long_array, vec![-2,1,-3,4,-1,2,1,-5,4] => 6);
    subarray_test!(test_single_array, vec![1] => 1);
    subarray_test!(test_short_array, vec![5,4,-1,7,8] => 23);
    subarray_test!(test_all_negative, vec![-2,-1] => -1);
    subarray_test!(test_tail_subarray, vec![0,-3,2,1,-2,3] => 4);
    subarray_test!(test_zig_subarray, vec![2,-1,2,1,3,-2,1,2,1,-2] => 9);
}