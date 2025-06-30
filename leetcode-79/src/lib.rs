use std::char;

pub struct Solution;

impl Solution {
    fn flat_match(
        string_list: &[char], // flattened vector
        row_size: usize,
        start: usize,
        word: &[char],
        index: usize,
        selected: &mut Vec<bool>,
    ) -> bool {
        if index == word.len() {
            return true;
        }

        let mut candidates = vec![];

        if start + 1 < string_list.len()
            && (start + 1) % row_size != 0
            && !selected[start + 1]
        {
            candidates.push(start + 1);
        }
        if start % row_size != 0 && !selected[start - 1] {
            candidates.push(start - 1);
        }
        if start >= row_size && !selected[start - row_size] {
            candidates.push(start - row_size);
        }
        if start + row_size < string_list.len() && !selected[start + row_size] {
            candidates.push(start + row_size);
        }

        for &next in candidates.iter() {
            if string_list[next] == word[index] {
                selected[next] = true;
                if Self::flat_match(
                    string_list,
                    row_size,
                    next,
                    word,
                    index + 1,
                    selected,
                ) {
                    return true;
                }
                selected[next] = false;
            }
        }

        false
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if word.is_empty() {
            return true;
        }

        let row_size = board[0].len();
        let flatten: Vec<char> = board.into_iter().flatten().collect();

        let start: Vec<usize> = flatten
            .iter()
            .enumerate()
            .filter(|(_i, c)| word.starts_with(**c))
            .map(|(i, _)| i)
            .collect();

        if start.is_empty() {
            return false;
        } else if word.len() == 1 {
            return true;
        }

        let mut selected = vec![false; flatten.len()];
        let word_vector: Vec<char> = word.chars().collect();

        for &start_index in start.iter() {
            selected[start_index] = true;
            if Self::flat_match(
                &flatten,
                row_size,
                start_index,
                &word_vector,
                1,
                &mut selected,
            ) {
                return true;
            }
            selected[start_index] = false;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! board_exist_test {
        ($name: ident, ($board: expr, $word: literal) => $output: literal) => {
            #[test]
            fn $name() {
                let word = $word.to_string();
                assert_eq!(Solution::exist($board, word), $output);
            }
        };
    }

    board_exist_test!(test_long_word, 
        (vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']],
         "ABCCED")
          => true);
    board_exist_test!(test_short_word, 
        (vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']],
         "SEE") 
         => true);

    board_exist_test!(test_false, 
        (vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']],
         "ABCB") 
         => false);
    board_exist_test!(test_small_board, (vec!["aa".chars().collect()], "aaa") => false);
    board_exist_test!(test_wrong_order, 
        (vec!["ab".chars().collect(), "cd".chars().collect()], "abcd") => false);
}
