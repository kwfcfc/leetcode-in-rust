

pub struct Solution;

impl Solution {
    fn backtrack(mut state: Vec<i32>, target: i32, start: usize, nums: &[i32], result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(state.clone());
            return;
        }

        for index in start..nums.len() {
            if target - nums[index] < 0 {
                return;
            }

            state.push(nums[index]);
            Self::backtrack(state.clone(), target - nums[index], index, nums, result);
            state.pop();
        }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut nums = candidates.clone();
        nums.sort();
        let mut result: Vec<Vec<i32>> = vec![];
        Self::backtrack(vec![], target, 0, &nums, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! combination_test {
        ($name: ident, ($input: expr, $target: literal) => $output: expr) => {
            #[test]
            fn $name() {
                let mut result = Solution::combination_sum($input, $target);
                result.iter_mut().for_each(|v| v.sort());
                result.sort();
                let mut expected = $output;
                expected.iter_mut().for_each(|v| v.sort());
                expected.sort();
                assert_eq!(result, expected);
            }
        };
    }

    combination_test!(test_long, (vec![6,3,2,6,7], 7) => vec![vec![2,2,3],vec![7]]);
    combination_test!(test_short, (vec![2,3,5], 8) => vec![vec![2,2,2,2],vec![2,3,3],vec![3,5]]);
    combination_test!(test_empty, (vec![2], 1) => Vec::<Vec<i32>>::new());
    combination_test!(test_wa, (vec![8,7,4,3], 11) => vec![vec![8,3],vec![7,4],vec![4,4,3]]);
    combination_test!(test_dup, (vec![3, 8, 4], 11) => vec![vec![3, 8], vec![3, 4, 4]]);
}
