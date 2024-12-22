use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

        nums.iter()
            .enumerate()
            .find_map(|(i, &num)| {
                let complement = target - num;
                if let Some(&j) = map.get(&complement) {
                    Some(vec![j as i32, i as i32])
                } else {
                    map.insert(num, i);
                    None
                }
            })
            .unwrap_or_else(Vec::new)
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
}
