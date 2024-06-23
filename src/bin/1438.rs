use std::collections::VecDeque;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut ans = 1;
        let mut min_q = VecDeque::new();
        let mut max_q = VecDeque::new();

        let mut l = 0;

        for r in 0..nums.len() {
            // Maintain the minQ in non-increasing order
            while !min_q.is_empty() && *min_q.back().unwrap() > nums[r] {
                min_q.pop_back();
            }
            min_q.push_back(nums[r]);

            // Maintain the maxQ in non-decreasing order
            while !max_q.is_empty() && *max_q.back().unwrap() < nums[r] {
                max_q.pop_back();
            }
            max_q.push_back(nums[r]);

            // Adjust the window by removing elements from the front if the difference exceeds limit
            while max_q.front().unwrap() - min_q.front().unwrap() > limit {
                if *min_q.front().unwrap() == nums[l] {
                    min_q.pop_front();
                }
                if *max_q.front().unwrap() == nums[l] {
                    max_q.pop_front();
                }
                l += 1;
            }

            // Update the maximum length of the valid subarray found so far
            ans = ans.max(r - l + 1);
        }

        ans as i32
    }
}


struct Solution;
fn main() {
    // Example usage:
    let nums = vec![8, 2, 4, 7];
    let limit = 4;
    let result = Solution::longest_subarray(nums, limit);
    println!("Longest subarray length: {}", result);
}
