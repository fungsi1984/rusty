use std::collections::HashMap;
impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Define an inner function (like a "local procedure" in Scheme)
        fn find_sum_indices(
            nums: &Vec<i32>,
            target: i32,
            seen: &mut HashMap<i32, usize>,
            index: usize,
        ) -> Option<Vec<i32>> {
            // Base case: If we've exhausted the list, no solution found
            if index >= nums.len() {
                return None;
            }

            let complement = target - nums[index];

            // Check if the complement exists in our hash map (constant time lookup)
            if let Some(&complement_index) = seen.get(&complement) {
                // Found the pair! Return the indices
                return Some(vec![complement_index as i32, index as i32]);
            } else {
                // Add the current number and its index to the hash map
                seen.insert(nums[index], index);

                // Recursively call the inner function, incrementing the index
                return find_sum_indices(nums, target, seen, index + 1);
            }
        }

        // Initialize the hash map to store seen numbers and their indices
        let mut seen: HashMap<i32, usize> = HashMap::new();

        // Call the inner recursive function starting from index 0
        match find_sum_indices(&nums, target, &mut seen, 0) {
            Some(result) => result, // Unwrap the Option if a result was found
            None => vec![],         // Return an empty vector if no solution
        }
    }
}

struct Solution;
fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let result = Solution::two_sum(nums, target);
    println!("{:?}", result);
}
