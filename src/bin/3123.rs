impl Solution {
    pub fn find_answer(n: i32, edges: Vec<Vec<i32>>) -> Vec<bool> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let (m, n) = (edges.len(), n as usize);
        let mut g = vec![vec![]; n];

        for (i, edges_i) in edges.iter().enumerate().take(m) {
            let (u, v) = (edges_i[0] as usize, edges_i[1] as usize);
            let w = edges[i][2];
            g[u].push((v, w, i));
            g[v].push((u, w, i));
        }

        let mut pq = BinaryHeap::new();
        let mut dist = vec![i64::MAX; n];
        pq.push(Reverse((0, 0)));
        while let Some(Reverse((d, u))) = pq.pop() {
            if d >= dist[u] {
                continue;
            }
            dist[u] = d;

            for (v, w, _i) in &g[u] {
                if dist[u] + *w as i64 >= dist[*v] {
                    continue;
                }
                pq.push(Reverse((dist[u] + *w as i64, *v)));
            }
        }

        let mut sk = vec![];
        let mut ret = vec![false; m];

        sk.push(n - 1);
        while let Some(u) = sk.pop() {
            if dist[u] == i64::MAX {
                continue;
            }
            for (v, w, i) in &g[u] {
                if dist[u] < dist[*v] + *w as i64 {
                    continue;
                }
                ret[*i] = true;
                sk.push(*v);
            }
        }

        ret
    }
}

struct Solution;

fn main() {
    let n = 6;
    let edges = vec![
        vec![0, 1, 5],
        vec![0, 2, 3],
        vec![1, 2, 2],
        vec![1, 3, 1],
        vec![2, 5, 1],
        vec![3, 4, 2],
        vec![4, 5, 4],
    ];

    let result = Solution::find_answer(n, edges);
    println!("{:?}", result); // Output: [true, false, true, true, true, true]
}
