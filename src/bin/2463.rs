use std::cmp::min;
use std::i64::MAX;

struct Solution;

impl Solution {
    pub fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        let mut robot = robot.clone();
        let mut factory = factory.clone();
        robot.sort();
        factory.sort_by(|a, b| a[0].cmp(&b[0]));
        
        let mut mem = vec![vec![vec![-1; robot.len()]; factory.len()]; robot.len()];
        Solution::min_distance(&robot, &factory, 0, 0, 0, &mut mem)
    }

    // Private helper function
    fn min_distance(
        robot: &[i32],
        factory: &[Vec<i32>],
        i: usize,
        j: usize,
        k: usize,
        mem: &mut Vec<Vec<Vec<i64>>>,
    ) -> i64 {
        if i == robot.len() {
            return 0;
        }
        if j == factory.len() {
            return MAX;
        }
        if mem[i][j][k] != -1 {
            return mem[i][j][k];
        }
        
        let skip_factory = Solution::min_distance(robot, factory, i, j + 1, 0, mem);
        
        let position = factory[j][0];
        let limit = factory[j][1] as usize;

        let use_factory = if k < limit {
            Solution::min_distance(robot, factory, i + 1, j, k + 1, mem) + (robot[i] - position).abs() as i64
        } else {
            MAX / 2
        };
        
        mem[i][j][k] = min(skip_factory, use_factory);
        mem[i][j][k]
    }
}

fn main() {
    let robot = vec![1, 3, 5, 6]; // Positions of robots
    let factory = vec![vec![2, 2], vec![4, 3]]; // [position, max capacity] for each factory

    let result = Solution::minimum_total_distance(robot, factory);
    println!("Minimum total distance: {}", result);
}