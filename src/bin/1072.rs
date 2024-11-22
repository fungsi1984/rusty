use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ans = 0;
        let mut seen = HashSet::new();

        for i in 0..m {
            if seen.contains(&i) {
                continue;
            }
            let mut count = 0;
            let mut flip = vec![0; n];
            for j in 0..n {
                flip[j] = 1 ^ matrix[i][j];
            }
            for k in 0..m {
                if matrix[k] == matrix[i] || matrix[k] == flip {
                    seen.insert(k);
                    count += 1;
                }
            }
            ans = ans.max(count);
        }

        ans
    }
}

fn main() {
    let matrix = vec![vec![0, 1], vec![1, 1]];
    let result = Solution::max_equal_rows_after_flips(matrix);
    println!("{}", result); // Output should be 1
}
