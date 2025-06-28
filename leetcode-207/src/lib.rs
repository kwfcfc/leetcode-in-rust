use std::{cmp::max, collections::VecDeque};

pub struct Solution;

impl Solution {
    fn adj_graph(v: i32, edges: Vec<Vec<i32>>) -> (Vec<i32>, Vec<Vec<i32>>) {
        let mut in_graph = vec![0; v as usize];
        let mut out_graph = vec![Vec::<i32>::new(); v as usize];

        for edge in edges.iter() {
            let (from, to) = (edge[0], edge[1]);
            out_graph[from as usize].push(to);
            in_graph[to as usize] += 1;
        }

        (in_graph, out_graph)
    }

    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let (mut in_graph, out_graph) = Solution::adj_graph(num_courses, prerequisites);

        let mut visited = 0;
        let mut queue: VecDeque<_> = in_graph
            .iter()
            .enumerate()
            .filter(|&(_i, degree)| *degree == 0)
            .map(|(i, _)| i as i32)
            .collect();

        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();

            for &out in out_graph[node as usize].iter() {
                in_graph[out as usize] = max(0, in_graph[out as usize] - 1);

                if in_graph[out as usize] == 0 {
                    queue.push_back(out);
                }
            }

            visited += 1;
        }

        visited == num_courses
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! prerequisites_tests {
        ($name: ident, ($num: literal, $prerequisites: expr) => $output: literal) => {
            #[test]
            fn $name() {
                let result: bool = Solution::can_finish($num, $prerequisites);
                assert_eq!(result, $output);
            }
        };
    }

    prerequisites_tests!(test_2_courses, (2, vec![vec![1,0]]) => true);
    prerequisites_tests!(test_cannot, (2, vec![vec![1,0],vec![0,1]]) => false);
}
