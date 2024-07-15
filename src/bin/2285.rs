impl Solution {
    pub fn maximum_importance(n: i32, mut roads: Vec<Vec<i32>>) -> i64 {
        let mut graph = roads
            .iter()
            .fold(vec![0 as i64; n as usize], |mut acc, road| {
                acc[road[0] as usize] += 1;
                acc[road[1] as usize] += 1;
                acc
            });

        graph.sort_unstable();
        let mut total = 0;
        for i in 1..=n {
            total += graph[i as usize - 1] * i as i64
        }
        total
    }
}

struct Solution;
fn main() {
    let n = 4;
    let roads = vec![vec![0, 1], vec![1, 2], vec![2, 3]];
    let result = Solution::maximum_importance(n, roads);
    println!("Maximum Importance: {}", result); // Output: 17
}
