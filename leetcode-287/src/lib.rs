pub struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let (mut fast, mut slow) = (nums[0] as usize, nums[0] as usize);

        loop {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
            if fast == slow {
                break;
            }
        }

       fast = nums[0] as usize;
        while fast != slow {
            fast = nums[fast] as usize;
            slow = nums[slow] as usize;
        }

        slow as i32
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[macro_export]
    macro_rules! find_duplicate_test {
        ($name: ident, $input: expr => $output: expr) => {
            #[test]
            fn $name() {
                let result = Solution::find_duplicate($input);
                assert_eq!(result, $output);
            }
        };
    }

    find_duplicate_test!(test_continuous, vec![1,3,4,2,2] => 2);
    find_duplicate_test!(test_non_continuous, vec![3,1,3,4,2] => 3);
    find_duplicate_test!(test_all_duplicates, vec![3,3,3,3,3] => 3);
}
