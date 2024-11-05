use std::cmp::min;

struct Solution;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut dp = matrix.clone();
        let rows = dp.len();
        let cols = dp[0].len();

        for i in 1..rows {
            for j in 1..cols {
                if dp[i][j] == 1 {
                    dp[i][j] = min(min(dp[i - 1][j - 1], dp[i - 1][j]), dp[i][j - 1]) + 1;
                }
            }
        }

        dp.iter().flatten().sum()
    }
}

fn main() {
    let matrix = vec![
        vec![0, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![0, 1, 1, 1],
    ];

    let result = Solution::count_squares(matrix);
    println!("The count of squares is: {}", result); // Output: 15
}
