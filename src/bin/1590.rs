use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let sum: i64 = nums.iter().map(|&x| x as i64).sum(); // sum as i64 to avoid overflow
        let remainder = (sum % p as i64) as i32; // cast to i32 after mod
        if remainder == 0 {
            return 0;
        }

        let mut prefix_to_index = HashMap::new();
        prefix_to_index.insert(0, -1); // map prefix 0 to index -1
        let mut ans = nums.len() as i32; // ans as i32 to match expected return type
        let mut prefix = 0;

        for (i, &num) in nums.iter().enumerate() {
            prefix = (prefix + num) % p; // update prefix mod p
            let target = (prefix - remainder + p) % p;

            if let Some(&idx) = prefix_to_index.get(&target) {
                ans = ans.min(i as i32 - idx);
            }

            prefix_to_index.insert(prefix, i as i32);
        }

        if ans == nums.len() as i32 {
            -1
        } else {
            ans
        }
    }
}

fn main() {
    // Example usage
    let nums = vec![3, 1, 4, 2];
    let p = 6;
    let result = Solution::min_subarray(nums, p);
    println!("Result: {}", result); // Expected output: 1

    // Another example
    let nums2 = vec![6, 3, 5, 2];
    let p2 = 9;
    let result2 = Solution::min_subarray(nums2, p2);
    println!("Result: {}", result2); // Expected output: 2
}
