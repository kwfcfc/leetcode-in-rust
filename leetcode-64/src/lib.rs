use std::cmp::min;

pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];

        dp[0][0] = grid[0][0];

        // pre fill the first row
        for i in 1..n {
            dp[0][i] = dp[0][i - 1] + grid[0][i];
        }

        // pre fill the first column
        for i in 1..m {
            dp[i][0] = dp[i - 1][0] + grid[i][0];
        }

        for i in 1..m {
            for j in 1..n {
                let left = dp[i][j - 1];
                let up = dp[i - 1][j];
                dp[i][j] = min(left, up) + grid[i][j];
            }
        }

        dp[m - 1][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! path_test {
        ($name: ident, $input: expr => $output: literal) => {
            #[test]
            fn $name() {
                let result = Solution::min_path_sum($input);
                assert_eq!(result, $output);
            }
        };
    }

    path_test!(test_square, vec![vec![1,3,1],vec![1,5,1],vec![4,2,1]] => 7);
    path_test!(test_rec, vec![vec![1,2,3],vec![4,5,6]] => 12);
}
