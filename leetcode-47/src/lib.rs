use std::collections::HashSet;

pub struct Solution;

impl Solution {
    fn backtrack(
        choices: &[i32],
        selected: &mut Vec<bool>,
        mut state: Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if state.len() == choices.len() {
            result.push(state);
            return;
        }

        let mut duplicate = HashSet::new();
        for i in 0..choices.len() {
            let choice = choices[i];
            if !selected[i] && !duplicate.contains(&choice) {
                state.push(choice);
                selected[i] = true;
                duplicate.insert(choice);
                Self::backtrack(choices, selected, state.clone(), result);
                selected[i] = false;
                state.pop();
            }
        }
    }

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut selected = vec![false; nums.len()];

        Self::backtrack(&nums, &mut selected, vec![], &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! permute_test {
        ($name: ident, $input: expr => $output: expr) => {
            #[test]
            fn $name() {
                let mut result = Solution::permute_unique($input);
                result.sort();
                let mut expected = $output;
                expected.sort();
                assert_eq!(result, expected);
            }
        };
    }

    permute_test!(test_duplicate, vec![1,1,2] => vec![vec![1,1,2],vec![1,2,1],vec![2,1,1]]);
    permute_test!(test_simple, vec![1,2,3] => vec![vec![1,2,3],vec![1,3,2],vec![2,1,3],
             vec![2,3,1],vec![3,1,2],vec![3,2,1]]);
}
