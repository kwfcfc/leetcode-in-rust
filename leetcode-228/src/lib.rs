pub struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        todo!();
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
