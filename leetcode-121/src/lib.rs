pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let mut buy_price = prices[0];
        let mut max_profit = 0;

        for i in 1..prices.len() {
            if prices[i] < buy_price {
                buy_price = prices[i]
            } else if prices[i] - buy_price > max_profit {
                max_profit = prices[i] - buy_price;
            }
        }

        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! profit_test {
        ($name: ident, $input: expr => $output: literal) => {
            #[test]
            fn $name() {
                let result = Solution::max_profit($input);
                assert_eq!(result, $output);
            }
        };
    }

    profit_test!(test_profit, vec![7,1,5,3,6,4] => 5);
    profit_test!(test_no_profit, vec![7,6,4,3,1] => 0);
}
