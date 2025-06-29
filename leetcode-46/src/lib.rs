pub struct Solution;

impl Solution {
    fn backtrack(nums: &[i32], mut state: Vec<i32>, choices: &mut Vec<bool>, result: &mut Vec<Vec<i32>>) -> () {
        if nums.len() == state.len() {
            result.push(state);
            return;
        }

        for choice in 0..choices.len() {
            if !choices[choice] {
                state.push(nums[choice]);
                choices[choice] = true;
                Self::backtrack(nums, state.clone(), choices, result);
                // set back to false in the next backtrack
                choices[choice] = false;
                state.pop();
            }
        }
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let length = nums.len();
        let mut choices = vec![false; length];
        let mut result: Vec<Vec<i32>> = vec![];

        Self::backtrack(&nums, vec![], &mut choices, &mut result);
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
                let mut result = Solution::permute($input);
                result.sort();
                let mut expected = $output;
                expected.sort();
                assert_eq!(result, expected);
            }
        };
    }

    permute_test!(test_simple, vec![1,2,3] => 
        vec![vec![1,2,3],vec![1,3,2],vec![2,1,3],
             vec![2,3,1],vec![3,1,2],vec![3,2,1]]);
    permute_test!(test_two, vec![0,1] => vec![vec![0,1],vec![1,0]]);
    permute_test!(test_only_one, vec![1] => vec![vec![1]]);
}
