use std::collections::{VecDeque, HashSet};

#[derive(Clone, Copy, PartialEq, Eq)]
enum State {
    Init,
    Visiting,
    Visited,
}

struct Solution;

impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let n = favorite.len();
        let mut sum_components_length = 0; // the component: a -> b -> c <-> x <- y
        let mut graph = vec![vec![]; n];
        let mut in_degrees = vec![0; n];
        let mut max_chain_length = vec![1; n];

        // Build the graph.
        for (i, &f) in favorite.iter().enumerate() {
            graph[i].push(f as usize);
            in_degrees[f as usize] += 1;
        }

        // Perform topological sorting.
        let mut q: VecDeque<usize> = in_degrees.iter()
            .enumerate()
            .filter(|&(_, &d)| d == 0)
            .map(|(i, _)| i)
            .collect();

        while let Some(u) = q.pop_front() {
            for &v in &graph[u] {
                in_degrees[v] -= 1;
                if in_degrees[v] == 0 {
                    q.push_back(v);
                }
                max_chain_length[v] = max_chain_length[v].max(1 + max_chain_length[u]);
            }
        }

        for i in 0..n {
            if favorite[favorite[i] as usize] as usize == i {
                // i <-> favorite[i] (the cycle's length = 2)
                sum_components_length += max_chain_length[i] + max_chain_length[favorite[i] as usize];
            }
        }

        let mut max_cycle_length = 0; // Cycle: a -> b -> c -> a
        let mut parent = vec![-1; n];
        let mut seen = HashSet::new();
        let mut states = vec![State::Init; n];

        fn find_cycle(
            u: usize,
            graph: &Vec<Vec<usize>>,
            parent: &mut Vec<i32>,
            seen: &mut HashSet<usize>,
            states: &mut Vec<State>,
            max_cycle_length: &mut i32,
        ) {
            seen.insert(u);
            states[u] = State::Visiting;
            for &v in &graph[u] {
                if !seen.contains(&v) {
                    parent[v] = u as i32;
                    find_cycle(v, graph, parent, seen, states, max_cycle_length);
                } else if states[v] == State::Visiting {
                    // Find the cycle's length.
                    let mut curr = u;
                    let mut cycle_length = 1;
                    while curr != v {
                        curr = parent[curr] as usize;
                        cycle_length += 1;
                    }
                    *max_cycle_length = (*max_cycle_length).max(cycle_length);
                }
            }
            states[u] = State::Visited;
        }

        for i in 0..n {
            if !seen.contains(&i) {
                find_cycle(i, &graph, &mut parent, &mut seen, &mut states, &mut max_cycle_length);
            }
        }

        (sum_components_length / 2).max(max_cycle_length)
    }
}

fn main() {
    let favorite = vec![2, 2, 1, 2];
    let result = Solution::maximum_invitations(favorite);
    println!("Maximum invitations: {}", result); // Output: 3
}