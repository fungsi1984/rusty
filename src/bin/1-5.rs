use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Map of number -> index.
        let mut num_map: HashMap<i32, usize> = HashMap::new();

        for i in 0..nums.len() {
            let num = nums[i];
            let complement = target - num;
            if let Some(c) = num_map.get(&complement) {
                if *c == i {
                    continue;
                }
                return vec![i as i32, *c as i32];
            }
            num_map.insert(num, i);
        }

        Vec::new()
    }
}

struct Solution;
fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!(
        "Indices of the two numbers that add up to {}: {:?}",
        target, result
    );
}
