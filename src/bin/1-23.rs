use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        fn find_pair<F>(nums: &Vec<i32>, target: i32, lookup_fn: F) -> Vec<i32>
        where
            F: Fn(&HashMap<i32, usize>, i32, usize) -> Option<Vec<i32>>,
        {
            let mut num_map: HashMap<i32, usize> = HashMap::new();

            for (index, &num) in nums.iter().enumerate() {
                if let Some(result) = lookup_fn(&num_map, target - num, index) {
                    return result;
                }
                num_map.insert(num, index);
            }

            vec![]
        }

        let lookup = |map: &HashMap<i32, usize>, complement: i32, current_index: usize| -> Option<Vec<i32>> {
            map.get(&complement)
                .map(|&complement_index| vec![complement_index as i32, current_index as i32])
        };

        find_pair(&nums, target, lookup)
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
}
