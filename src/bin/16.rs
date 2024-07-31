use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        // Sort the array
        nums.sort();

        // Initialize the answer with the sum of the first three numbers
        let mut ans = nums[0] + nums[1] + nums[2];

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if sum == target {
                    return sum;
                }
                if (sum - target).abs() < (ans - target).abs() {
                    ans = sum;
                }
                match sum.cmp(&target) {
                    Ordering::Less => l += 1,
                    Ordering::Greater => r -= 1,
                    _ => {}
                }
            }
        }

        ans
    }
}

// fn main() {
//     // Define some test cases directly as Vec<i32>
//     let test_cases = vec![
//         (vec![-1, 2, 1, -4], 1),
//         (vec![0, 2, 1, -3], 1),
//         (vec![-1, 2, 1, -4], 1),
//         (vec![1, 1, 1, 0], -100),
//         (vec![1, 1, 1, 0], 100),
//         (vec![-1, 0, 1, 1, 55], 3),
//     ];

//     // Run the test cases
//     for (nums, target) in test_cases {
//         // Process nums by consuming it
//         let result = Solution::three_sum_closest(nums.clone(), target);
//         println!(
//             "The sum closest to {} for array {:?} is: {}",
//             target, nums, result
//         );
//     }
// }

fn main() {
    // Define some test cases directly as Vec<i32>
    let test_cases = vec![
        (vec![-1, 2, 1, -4], 1),
        (vec![0, 2, 1, -3], 1),
        (vec![-1, 2, 1, -4], 1),
        (vec![1, 1, 1, 0], -100),
        (vec![1, 1, 1, 0], 100),
        (vec![-1, 0, 1, 1, 55], 3),
    ];

    // Run the test cases
    for (nums, target) in test_cases {
        // Process nums by borrowing it
        let result = Solution::three_sum_closest(nums.to_vec(), target);
        println!(
            "The sum closest to {} for array {:?} is: {}",
            target, nums, result
        );
    }
}
