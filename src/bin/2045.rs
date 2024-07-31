use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let n = n as usize;
        let mut adj = vec![Vec::new(); n + 1];

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            adj[u].push(v);
            adj[v].push(u);
        }

        let mut dist1 = vec![i32::MAX; n + 1];
        let mut dist2 = vec![i32::MAX; n + 1];
        let mut freq = vec![0; n + 1];

        let mut min_heap = BinaryHeap::new();
        min_heap.push(Reverse((0, 1))); // (time, node)
        dist1[1] = 0;

        while let Some(Reverse((mut time_taken, node))) = min_heap.pop() {
            freq[node] += 1;

            if freq[node] == 2 && node == n {
                return time_taken;
            }

            if (time_taken / change) % 2 != 0 {
                time_taken = change * (time_taken / change + 1) + time;
            } else {
                time_taken += time;
            }

            for &neighbor in &adj[node] {
                if freq[neighbor] == 2 {
                    continue;
                }

                if dist1[neighbor] > time_taken {
                    dist2[neighbor] = dist1[neighbor];
                    dist1[neighbor] = time_taken;
                    min_heap.push(Reverse((time_taken, neighbor)));
                } else if dist2[neighbor] > time_taken && dist1[neighbor] != time_taken {
                    dist2[neighbor] = time_taken;
                    min_heap.push(Reverse((time_taken, neighbor)));
                }
            }
        }

        0
    }
}

fn main() {
    // Example usage:
    let n = 5;
    let edges = vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 4], vec![4, 5]];
    let time = 3;
    let change = 5;
    let result = Solution::second_minimum(n, edges, time, change);
    println!("Second minimum time: {}", result);
}
