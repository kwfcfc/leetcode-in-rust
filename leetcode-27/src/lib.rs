pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let filtered: Vec<i32> = nums.iter().cloned().filter(|x| *x != val).collect();

        let k = filtered.len();

        nums[..k].copy_from_slice(&filtered);
        return k as i32;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn compare_vec_unordered(a: &[i32], b: &[i32]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut count_a = HashMap::new();
        let mut count_b = HashMap::new();

        for num in &a[..b.len()] {
            *count_a.entry(num).or_insert(0) += 1;
        }

        for num in b {
            *count_b.entry(num).or_insert(0) += 1;
        }

        count_a == count_b
    }

    macro_rules! remove_test {
        ($name: ident, ($original: expr, $value: expr) => ($expected: expr, $result: expr)) => {
            #[test]
            fn $name() {
                let mut input = $original;
                let output = $expected;

                let result = Solution::remove_element(&mut input, $value);
                assert_eq!($result, result);

                let compare = compare_vec_unordered(&output, &input[..$result]);
                assert_eq!(true, compare);
            }
        };
    }

    remove_test!(test_simple, (vec![3, 2, 2, 3], 3) => (vec![2, 2], 2));
    remove_test!(test_duplicate, (vec![0,1,2,2,3,0,4,2], 2) => (vec![0,1,4,0,3], 5));
    remove_test!(test_223, (vec![2,2,3], 2) => (vec![3], 1));
}
