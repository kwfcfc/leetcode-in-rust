pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let length = nums.len();

        if length == 1 {
            return nums[0];
        }

        let mut previous = nums[0];
        let mut neighbor = if nums[1] > previous {
            nums[1]
        } else {
            previous
        };

        for &num in nums.iter().skip(2) {
            let temp = neighbor;
            if neighbor < previous + num {
                neighbor = previous + num;
            }
            previous = temp;
        }

        neighbor
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

    rob_test!(test_max, vec![1,2,3,1] => 4);
    rob_test!(test_only_two, vec![2,7,9,3,1] => 12);
    rob_test!(test_head_and_tail, vec![2,1,1,2] => 4);
}
