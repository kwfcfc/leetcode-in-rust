use std::char;

pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn dfs(
            board: &mut Vec<Vec<char>>,
            word: &[u8],
            index: usize,
            pos: (usize, usize),
            boundary: (usize, usize),
        ) -> bool {
            let ((i, j), (m, n)) = (pos, boundary);
            if board[i][j] != word[index] as char {
                return false;
            }
            if index == word.len() - 1 {
                return true;
            }
            board[i][j] = '\0'; // mark as visited
            for (x, y) in [
                (i, j.saturating_sub(1)),
                (i, j + 1),
                (i.saturating_sub(1), j),
                (i + 1, j),
            ] {
                if x < m
                    && y < n
                    && dfs(board, word, index + 1, (x, y), boundary)
                {
                    return true;
                }
            }

            board[i][j] = word[index] as char;
            false // not found
        }

        let map_alphabet_to_ascii = |c: u8| {
            if c.is_ascii_lowercase() {
                Some(c as usize - 'a' as usize)
            } else if c.is_ascii_uppercase() {
                Some(c as usize - 'A' as usize + 26)
            } else {
                None
            }
        };

        // optimization 1: count the word frequency
        let mut char_count = [0u8; 52]; // 26 uppercase and 26 lowercase English word
        for r in &board {
            for &c in r {
                if let Some(index) = map_alphabet_to_ascii(c as u8) {
                    char_count[index] += 1;
                }
            }
        }

        let mut word_count = [0u8; 52];
        let word_bytes = word.as_bytes();
        for &c in word.as_bytes() {
            if let Some(index) = map_alphabet_to_ascii(c) {
                word_count[index] += 1;
                // return false if there is not enough char in the graph
                if word_count[index] > char_count[index] {
                    return false;
                }
            }
        }

        // Optimization II: count which way has less frequency, normal or reverse
        let word_mut = if let (Some(start), Some(end)) = (
            map_alphabet_to_ascii(word_bytes[0]),
            map_alphabet_to_ascii(word_bytes[word_bytes.len() - 1]),
        ) {
            if start > end {
                word.chars().rev().collect()
            } else {
                word.clone()
            }
        } else {
            word.clone()
        };

        let mut mutable = board.clone();
        let boundary = (board.len(), board[0].len());

        for i in 0..boundary.0 {
            for j in 0..boundary.1 {
                if dfs(&mut mutable, word_mut.as_bytes(), 0, (i, j), boundary) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
#[rustfmt::skip]
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
