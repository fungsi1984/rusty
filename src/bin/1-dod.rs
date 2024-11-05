use std::collections::HashMap;

struct TwoSumData<'a> {
    nums: &'a [i32],
    target: i32,
    map: HashMap<i32, usize>,
}

impl<'a> TwoSumData<'a> {
    fn new(nums: &'a [i32], target: i32) -> Self {
        Self {
            nums,
            target,
            map: HashMap::with_capacity(nums.len()),
        }
    }

    fn find_indices(&mut self) -> Vec<i32> {
        for (i, &value) in self.nums.iter().enumerate() {
            let complement = self.target - value;
            if let Some(&index) = self.map.get(&complement) {
                return vec![index as i32, i as i32];
            }
            self.map.insert(value, i);
        }
        vec![]
    }
}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut data = TwoSumData::new(&nums, target);
        data.find_indices()
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

struct Solution;
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