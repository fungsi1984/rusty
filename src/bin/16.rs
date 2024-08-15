use std::cmp::Ordering;

struct Solution;

// impl Solution {
//     pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
//         // Sort the array
//         nums.sort();

//         // Initialize the answer with the sum of the first three numbers
//         let mut ans = nums[0] + nums[1] + nums[2];

//         for i in 0..nums.len() - 2 {
//             if i > 0 && nums[i] == nums[i - 1] {
//                 continue;
//             }
//             let mut l = i + 1;
//             let mut r = nums.len() - 1;
//             while l < r {
//                 let sum = nums[i] + nums[l] + nums[r];
//                 if sum == target {
//                     return sum;
//                 }
//                 if (sum - target).abs() < (ans - target).abs() {
//                     ans = sum;
//                 }
//                 match sum.cmp(&target) {
//                     Ordering::Less => l += 1,
//                     Ordering::Greater => r -= 1,
//                     _ => {}
//                 }
//             }
//         }

//         ans
//     }
// }

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        // Sort the array
        let mut nums = nums;
        nums.sort_unstable();

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

// painfull c++

// #include <iostream>
// #include <vector>
// #include <algorithm>
// #include <cmath>
// #include <limits>

// class Solution {
// public:
//     int threeSumClosest(const std::vector<int>& nums, int target) {
//         int n = nums.size();
//         int ans = nums[0] + nums[1] + nums[2];

//         std::vector<int> sorted_nums = nums; // Make a copy of nums and sort it
//         std::sort(sorted_nums.begin(), sorted_nums.end());

//         for (int i = 0; i < n - 2; ++i) {
//             if (i > 0 && sorted_nums[i] == sorted_nums[i - 1]) {
//                 continue; // Skip duplicates
//             }
//             int l = i + 1;
//             int r = n - 1;
//             while (l < r) {
//                 int summ = sorted_nums[i] + sorted_nums[l] + sorted_nums[r];
//                 if (summ == target) {
//                     return summ;
//                 }
//                 if (std::abs(summ - target) < std::abs(ans - target)) {
//                     ans = summ;
//                 }
//                 if (summ < target) {
//                     ++l;
//                 } else {
//                     --r;
//                 }
//             }
//         }

//         return ans;
//     }
// };

// why even we use std::pair
// int main() {
//     std::vector<std::pair<std::vector<int>, int>> test_cases = {
//         {{ -1, 2, 1, -4 }, 1},
//         {{ 0, 2, 1, -3 }, 1},
//         {{ -1, 2, 1, -4 }, 1},
//         {{ 1, 1, 1, 0 }, -100},
//         {{ 1, 1, 1, 0 }, 100},
//         {{ -1, 0, 1, 1, 55 }, 3},
//     };

//     Solution sol;
//     for (const auto& [nums, target] : test_cases) {
//         int result = sol.threeSumClosest(nums, target);
//         std::cout << "The sum closest to " << target << " for array ";
//         for (int num : nums) {
//             std::cout << num << " ";
//         }
//         std::cout << "is: " << result << std::endl;
//     }

//     return 0;
// }

// in java we need to build pair function

// import java.util.Arrays;

// class Pair {
//     int[] nums;
//     int target;

//     Pair(int[] nums, int target) {
//         this.nums = nums;
//         this.target = target;
//     }
// }

// class Solution {
//     public int threeSumClosest(int[] nums, int target) {
//         int ans = nums[0] + nums[1] + nums[2];

//         Arrays.sort(nums);

//         for (int i = 0; i < nums.length - 2; i++) {
//             if (i > 0 && nums[i] == nums[i - 1]) {
//                 continue;
//             }
//             int l = i + 1;
//             int r = nums.length - 1;
//             while (l < r) {
//                 int summ = nums[i] + nums[l] + nums[r];
//                 if (summ == target) {
//                     return summ;
//                 }
//                 if (Math.abs(summ - target) < Math.abs(ans - target)) {
//                     ans = summ;
//                 }
//                 if (summ < target) {
//                     l++;
//                 } else {
//                     r--;
//                 }
//             }
//         }

//         return ans;
//     }

// }

// class Main {
//     public static void main(String[] args) {
//         Solution solution = new Solution();

//         // Define test cases using Pair objects
//         Pair[] testCases = {
//             new Pair(new int[]{-1, 2, 1, -4}, 1),
//             new Pair(new int[]{0, 2, 1, -3}, 1),
//             new Pair(new int[]{-1, 2, 1, -4}, 1),
//             new Pair(new int[]{1, 1, 1, 0}, -100),
//             new Pair(new int[]{1, 1, 1, 0}, 100),
//             new Pair(new int[]{-1, 0, 1, 1, 55}, 3)
//         };

//         // Run the test cases
//         for (Pair testCase : testCases) {
//             int result = solution.threeSumClosest(testCase.nums, testCase.target);
//             System.out.println("The sum closest to " + testCase.target + " for array " + Arrays.toString(testCase.nums) + " is: " + result);
//         }
//     }
// }
