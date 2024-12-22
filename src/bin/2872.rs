use std::collections::VecDeque;

impl Solution {
    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
        let n = n as usize;
        let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];

        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut ans = 0;
        Self::dfs(&graph, 0, -1, &values, k, &mut ans);
        ans
    }

    fn dfs(graph: &Vec<Vec<usize>>, u: usize, prev: i32, values: &Vec<i32>, k: i32, ans: &mut i32) -> i64 {
        let mut tree_sum = values[u] as i64;

        for &v in &graph[u] {
            if v as i32 != prev {
                tree_sum += Self::dfs(graph, v, u as i32, values, k, ans);
            }
        }

        if tree_sum % k as i64 == 0 {
            *ans += 1;
        }
        tree_sum
    }
}

struct Solution;
// fn main() {
//     let n = 5;
//     let edges = vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]];
//     let values = vec![1, 8, 1, 4, 4];
//     let k = 6;

//     let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
//     for edge in &edges {
//         let u = edge[0] as usize;
//         let v = edge[1] as usize;
//         graph[u].push(v);
//         graph[v].push(u);
//     }

//     let mut visited = vec![false; n];
//     let mut ans = 0;

//     for i in 0..n {
//         if !visited[i] {
//             let mut component_nodes = Vec::new();
//             let mut stack = VecDeque::new();
//             stack.push_back(i);
//             visited[i] = true;

//             while let Some(u) = stack.pop_front() {
//                 component_nodes.push(u);
//                 for &v in &graph[u] {
//                     if !visited[v] {
//                         visited[v] = true;
//                         stack.push_back(v);
//                     }
//                 }
//             }

//             let component_sum: i32 = component_nodes.iter().map(|&x| values[x]).sum();

//             if component_sum % k == 0 {
//                 ans += 1;
//             }
//         }
//     }

//     println!("{}", ans); // Output: 2
// }

fn main() {
    let n = 5;
    let edges = vec![vec![0, 2], vec![1, 2], vec![1, 3], vec![2, 4]];
    let values = vec![1, 8, 1, 4, 4];
    let k = 6;

    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for edge in &edges {
        let u = edge[0] as usize;
        let v = edge[1] as usize;
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut visited = vec![false; n];
    let mut ans = 0;

    for i in 0..n {
        if !visited[i] {
            let mut component_sum = 0; // Initialize sum for EACH component
            let mut stack = VecDeque::new();
            stack.push_back(i);
            visited[i] = true;

            while let Some(u) = stack.pop_front() {
                component_sum += values[u]; // Add value of current node to component sum
                for &v in &graph[u] {
                    if !visited[v] {
                        visited[v] = true;
                        stack.push_back(v);
                    }
                }
            }

            if component_sum % k == 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans); // Output: 2
}