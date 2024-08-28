struct Solution;

impl Solution {
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        for i in 0..grid2.len() {
            for j in 0..grid2[0].len() {
                if grid2[i][j] == 1 {
                    ans += Self::dfs(&grid1, &mut grid2, i as i32, j as i32);
                }
            }
        }

        ans
    }

    fn dfs(grid1: &Vec<Vec<i32>>, grid2: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
        if i < 0 || i >= grid1.len() as i32 || j < 0 || j >= grid2[0].len() as i32 {
            return 1;
        }
        if grid2[i as usize][j as usize] != 1 {
            return 1;
        }

        grid2[i as usize][j as usize] = 2; // Mark 2 as visited.

        let up = Self::dfs(grid1, grid2, i + 1, j);
        let down = Self::dfs(grid1, grid2, i - 1, j);
        let right = Self::dfs(grid1, grid2, i, j + 1);
        let left = Self::dfs(grid1, grid2, i, j - 1);

        up & down & right & left & grid1[i as usize][j as usize]
    }
}

fn main() {
    let grid1 = vec![
        vec![1, 1, 1, 0, 0],
        vec![0, 1, 1, 0, 1],
        vec![0, 0, 0, 1, 1],
        vec![1, 1, 0, 0, 0],
        vec![0, 1, 0, 1, 1],
    ];

    let grid2 = vec![
        vec![1, 1, 1, 0, 0],
        vec![0, 0, 1, 0, 1],
        vec![0, 1, 0, 1, 1],
        vec![1, 0, 0, 0, 0],
        vec![0, 1, 0, 1, 1],
    ];

    let result = Solution::count_sub_islands(grid1, grid2);
    println!("Number of sub-islands: {}", result);
}
