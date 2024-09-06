struct Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let target_sum = (rolls.len() as i32 + n) * mean;
        let missing_sum: i32 = target_sum - rolls.iter().sum::<i32>();

        if missing_sum > n * 6 || missing_sum < n {
            return vec![];
        }

        let mut ans = vec![missing_sum / n; n as usize];
        let mut remainder = missing_sum % n;

        for i in 0..remainder as usize {
            ans[i] += 1;
        }

        ans
    }
}

fn main() {
    let rolls = vec![3, 2, 4, 3];
    let mean = 4;
    let n = 2;
    let result = Solution::missing_rolls(rolls, mean, n);
    println!("{:?}", result); // Example output: [6, 6]
}
