pub struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let total_sum: i32 = nums.iter().sum();

        let mut left_sum = 0;

        for (i, &num) in nums.iter().enumerate() {
            if left_sum == total_sum - left_sum - num {
                return i.try_into().unwrap();
            }
            left_sum += num
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! pivot_test {
        ($name: ident, $input: expr => $output: expr) => {
            #[test]
            fn $name() {
                let result = Solution::pivot_index($input);
                assert_eq!(result, $output);
            }
        };
    }

    pivot_test!(test_1, vec![1,7,3,6,5,6] => 3);
    pivot_test!(test_negative, vec![1,2,3] => -1);

    pivot_test!(test_leftmost, vec![2,1,-1] => 0);
}
