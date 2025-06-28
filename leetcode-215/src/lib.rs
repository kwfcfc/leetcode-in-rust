use std::{cmp::Reverse, collections::BinaryHeap};

pub struct Solution;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 0 {
            return i32::MIN;
        };
        let mut min_heap: BinaryHeap<Reverse<i32>> = nums
            .iter()
            .take(k.try_into().unwrap())
            .map(|&num| Reverse(num))
            .collect();

        for &num in nums.iter().skip(k.try_into().unwrap()) {
            if num > min_heap.peek().unwrap().0 {
                min_heap.pop();
                min_heap.push(Reverse(num));
            }
        }

        min_heap.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! vector_find_test {
        ($name: ident, ($input: expr, $k: literal) => $output: expr) => {
            #[test]
            fn $name() {
                let result = Solution::find_kth_largest($input, $k);
                assert_eq!(result, $output);
            }
        };
    }

    vector_find_test!(test_simple, (vec![3,2,1,5,6,4], 2) => 5);
    vector_find_test!(test_larger, (vec![3,2,3,1,2,4,5,5,6], 4) => 4);
}
