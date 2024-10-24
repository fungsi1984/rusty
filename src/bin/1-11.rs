use std::collections::HashMap;

struct Solution;

impl Solution {
    fn two_sum(nums: &[i32], target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            
            if let Some(&index) = map.get(&complement) {
                return vec![index, i as i32];
            }
            
            map.insert(num, i as i32);
        }
        
        vec![]
    }
}

fn print_example(test_case: (&[i32], i32)) {
    let (nums, target) = test_case;
    
    println!("\nInput: nums = {:?}, target = {}", nums, target);
    
    let result = Solution::two_sum(nums, target);
    
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
    let test_cases = [
        (&[2, 7, 11, 15][..], 9),
        (&[3, 2, 4][..], 6),
        (&[3, 3][..], 6),
        (&[1, 2, 3][..], 7),
    ];
    
    for test_case in test_cases {
        print_example(test_case);
    }
}