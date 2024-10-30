use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        
        match nums.iter()
            .enumerate()
            .find_map(|(i, &num)| {
                let complement = target - num;
                map.get(&complement)
                    .map(|&j| vec![j, i as i32])
                    .or_else(|| {
                        map.insert(num, i as i32);
                        None
                    })
            }) {
                Some(result) => result,
                None => vec![]
            }
    }
}

fn print_example(nums: Vec<i32>, target: i32) {
    println!("\nInput: nums = {:?}, target = {}", nums, target);
    
    let result = Solution::two_sum(nums.clone(), target);
    
    if !result.is_empty() {
        println!("Output: {:?}", result);
        println!(
            "Explanation: nums[{}] + nums[{}] = {} + {} = {}",
            result[0], result[1],
            nums[result[0] as usize], nums[result[1] as usize],
            target
        );
    } else {
        println!("No solution found");
    }
}

fn main() {
    let test_cases = vec![
        (vec![2, 7, 11, 15], 9),
        (vec![3, 2, 4], 6),
        (vec![3, 3], 6),
        (vec![1, 2, 3], 7),
    ];
    
    for (nums, target) in test_cases {
        print_example(nums, target);
    }
}