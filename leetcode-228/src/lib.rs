use std::vec;

pub struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return vec![];
        }
        if nums.len() == 1 {
            return vec![format!("{}", nums[0])];
        }
        let mut result = vec![];
        let mut start = nums[0];
        let mut end = start;
        for num in &nums[1..] {
            if end + 1 == *num {
                end += 1;
                continue;
            } 
            if end == start {
                result.push(format!("{start}"));
            } else {
                result.push(format!("{start}->{end}"));
            }
            start = *num;
            end = start;
        }
        if end == start {
            result.push(format!("{start}"));
        } else {
            result.push(format!("{start}->{end}"));
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[macro_export]
    macro_rules! range_test {
        ($name: ident, $input: expr => $output: expr) => {
            #[test]
            fn $name() {
                let result = Solution::summary_ranges($input);
                assert_eq!(result, $output);
            }
        };
    }

    range_test!(test_zero,vec![0] => vec!["0"]);
    range_test!(test_continuous,vec![0,1,2,3,4] => vec!["0->4"]);
    range_test!(
        test_simple,
        vec![0, 1, 2, 4, 5, 7] =>
        vec!["0->2", "4->5", "7"]
    );
    range_test!(
        test_2,
        vec![0, 2, 3, 4, 6, 8, 9] =>
        vec!["0", "2->4", "6", "8->9"]
    );
}
