use std::collections::VecDeque;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut ans = 1;
        let mut min_q = VecDeque::new();
        let mut max_q = VecDeque::new();

        let mut l = 0;

        for r in 0..nums.len() {
            // Maintain the minQ in non-increasing order
            while let Some(&back) = min_q.back() {
                if back > nums[r] {
                    min_q.pop_back();
                } else {
                    break;
                }
            }
            min_q.push_back(nums[r]);

            // Maintain the maxQ in non-decreasing order
            while let Some(&back) = max_q.back() {
                if back < nums[r] {
                    max_q.pop_back();
                } else {
                    break;
                }
            }
            max_q.push_back(nums[r]);

            // Adjust the window by removing elements from the front if the difference exceeds limit
            while let (Some(&max_front), Some(&min_front)) = (max_q.front(), min_q.front()) {
                if max_front - min_front > limit {
                    if min_front == nums[l] {
                        min_q.pop_front();
                    }
                    if max_front == nums[l] {
                        max_q.pop_front();
                    }
                    l += 1;
                } else {
                    break;
                }
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
