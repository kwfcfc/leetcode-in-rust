pub struct Solution;

impl Solution {
    fn backtrack(
        nums: &[i32],
        target: i32,
        mut state: Vec<i32>, // a possible total sum vector
        start: usize, // check the backtrack index in a sorted result
        result: &mut Vec<Vec<i32>>,
    ) {
        // the total sum found, push the vector back in the result
        if target == 0 {
            result.push(state.clone());
        }

        for index in start..nums.len() {
            let choice = nums[index];
            if target < choice {
                return;
            }

            // search and skip the duplicate number
            if index > start && choice == nums[index - 1] {
                continue;
            }
            state.push(choice);
            Self::backtrack(nums, target - choice, state.clone(), index + 1, result);
            state.pop();
        }
    }

    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut sorted = candidates.clone();
        sorted.sort();

        let mut result = Vec::<Vec<i32>>::new();
        Self::backtrack(&sorted, target, vec![], 0, &mut result);
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
                let mut result = Solution::combination_sum2($input, $target);
                result.iter_mut().for_each(|v| v.sort());
                result.sort();
                let mut expected = $output;
                expected.iter_mut().for_each(|v| v.sort());
                expected.sort();
                assert_eq!(result, expected);
            }
        };
    }

    combination_test!(test_long, (vec![10,1,2,7,6,1,5], 8) => vec![
    vec![1,1,6],
    vec![1,2,5],
    vec![1,7],
    vec![2,6]
    ]);

    combination_test!(test_short, (vec![2,5,2,1,2], 5) => vec![
    vec![1,2,2],
    vec![5]
    ]);
}
