impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let n = matrix.len();
        let m = matrix[0].len();

        let mut row_min = vec![i32::MAX; n];
        for i in 0..n {
            for j in 0..m {
                row_min[i] = row_min[i].min(matrix[i][j]);
            }
        }

        let mut col_max = vec![i32::MIN; m];
        for i in 0..m {
            for j in 0..n {
                col_max[i] = col_max[i].max(matrix[j][i]);
            }
        }

        let mut lucky_numbers = vec![];
        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == row_min[i] && matrix[i][j] == col_max[j] {
                    lucky_numbers.push(matrix[i][j]);
                }
            }
        }

        lucky_numbers
    }
}
struct Solution;
fn main() {
    let matrix = vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]];
    let lucky_numbers = Solution::lucky_numbers(matrix);

    println!("Lucky numbers: {:?}", lucky_numbers);
}
