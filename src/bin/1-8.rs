use std::collections::HashMap;

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            match map.get(&complement) {
                Some(&index) => return vec![index as i32, i as i32],
                None => map.insert(num, i),
            };
        }

        vec![] // This line is just a fallback and should never be reached given the problem constraints.
    }
}

struct Solution;
fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result); // Output: [0, 1]
}