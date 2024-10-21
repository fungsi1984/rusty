use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        
        // Closure to find two sum
        let mut find_two_sum = |num: i32, index: usize| -> Option<Vec<i32>> {
            let complement = target - num;
            if let Some(&complement_index) = map.get(&complement) {
                return Some(vec![complement_index as i32, index as i32]);
            }
            map.insert(num, index);
            None
        };
        
        // Iterate over nums and apply the closure
        for (i, &num) in nums.iter().enumerate() {
            if let Some(result) = find_two_sum(num, i) {
                return result;
            }
        }
        
        panic!("No two sum solution");
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);  // Output: [0, 1]
}
