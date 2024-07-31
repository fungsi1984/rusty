struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, current) in nums.iter().enumerate() {
            let i = i as i32;
            let result = target - current;
            if map.contains_key(&result) {
                return vec![map[&result], i];
            }

            map.insert(current, i);
        }
        panic!("Indexüberschreitung!");
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!(
        "Indices of the two numbers that add up to {}: {:?}",
        target, result
    );
}

#[test]
fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
