use std::collections::HashSet;
use std::cmp::max;

struct Solution;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
        // Step 1: Remove duplicates by converting to a HashSet
        let mut nums: Vec<i32> = nums.into_iter().collect::<HashSet<_>>().into_iter().collect();

        // Step 2: Sort the vector in descending order
        nums.sort_by(|a, b| b.cmp(a));

        // Step 3: Find the maximum number in the array
        let max_num = *nums.iter().max().unwrap();

        // Step 4: Create a DP array of size max_num + 1
        let mut dp = vec![0; (max_num + 1) as usize];

        // Step 5: Fill the DP array
        for &num in &nums {
            dp[num as usize] = 1;
            
            let squared_num = (num as i64) * (num as i64);
            if squared_num <= max_num as i64 {
                dp[num as usize] += dp[squared_num as usize];
            }
        }

        // Step 6: Find the maximum streak
        let ans = *dp.iter().max().unwrap();
        if ans < 2 { -1 } else { ans }
    }
}

fn main() {
    let nums = vec![16, 4, 2, 1, 4, 1]; // Example usage
    let result = Solution::longest_square_streak(nums);
    println!("The longest square streak is: {}", result);
}
