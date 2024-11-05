impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = 0;
        let mut dp = vec![vec![0; n]; m];

        for j in (0..n - 1).rev() {
            for i in 0..m {
                if grid[i][j + 1] > grid[i][j] {
                    dp[i][j] = dp[i][j + 1] + 1;
                }
                if i > 0 && grid[i - 1][j + 1] > grid[i][j] {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j + 1] + 1);
                }
                if i + 1 < m && grid[i + 1][j + 1] > grid[i][j] {
                    dp[i][j] = dp[i][j].max(dp[i + 1][j + 1] + 1);
                }
            }
        }

        for i in 0..m {
            ans = ans.max(dp[i][0]);
        }

        ans
    }
}

struct Solution;
fn main() {
    let grid = vec![
        vec![1, 3, 1],
        vec![3, 4, 5],
        vec![2, 3, 1],
    ];
    let result = Solution::max_moves(grid);
    println!("Maximum moves: {}", result);
}