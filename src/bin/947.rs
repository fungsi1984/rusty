use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut num_of_islands = 0;
        let mut graph = vec![vec![]; stones.len()];
        let mut seen = HashSet::new();

        for i in 0..stones.len() {
            for j in i + 1..stones.len() {
                if stones[i][0] == stones[j][0] || stones[i][1] == stones[j][1] {
                    graph[i].push(j);
                    graph[j].push(i);
                }
            }
        }

        for i in 0..stones.len() {
            if seen.insert(i) {
                Self::dfs(&graph, i, &mut seen);
                num_of_islands += 1;
            }
        }

        (stones.len() as i32) - num_of_islands
    }

    fn dfs(graph: &Vec<Vec<usize>>, u: usize, seen: &mut HashSet<usize>) {
        for &v in &graph[u] {
            if seen.insert(v) {
                Self::dfs(graph, v, seen);
            }
        }
    }
}

fn main() {
    let stones = vec![
        vec![0, 0],
        vec![0, 1],
        vec![1, 0],
        vec![1, 2],
        vec![2, 1],
        vec![2, 2],
    ];
    println!("{}", Solution::remove_stones(stones));
}
