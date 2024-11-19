use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        fn find_two_sum(
            index: usize,
            nums: &Vec<i32>,
            target: i32,
            map: &mut HashMap<i32, usize>,
        ) -> Vec<i32> {
            match index {
                // Base case: if the index goes out of bounds
                i if i >= nums.len() => vec![],

                // Recursive case: process the current number
                i => {
                    let complement = target - nums[i];

                    
                    match map.get(&complement) {
                        // If the complement is found, return the indices
                        Some(&complement_index) => {
  
                            vec![complement_index as i32, i as i32]
                        }

                        // Otherwise, insert the current number and continue recursion
                        None => {
                            map.insert(nums[i], i);
                            find_two_sum(i + 1, nums, target, map)
                        }
                    }
                }
            }
        }

        // Start recursion from index 0
        find_two_sum(0, &nums, target, &mut map)
    }
}

fn print_example(nums: Vec<i32>, target: i32) {
    println!("\nInput: nums = {:?}, target = {}", nums, target);

    let result = Solution::two_sum(nums.clone(), target);

    if !result.is_empty() {
        println!("Output: {:?}", result);
        println!(
            "Explanation: nums[{}] + nums[{}] = {} + {} = {}",
            result[0],
            result[1],
            nums[result[0] as usize],
            nums[result[1] as usize],
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
