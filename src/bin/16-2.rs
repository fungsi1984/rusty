use std::cmp::Ordering;
struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut sorted_nums = nums;
        sorted_nums.sort();

        let mut ans = sorted_nums[0] + sorted_nums[1] + sorted_nums[2];

        for i in 0..n - 2 {
            if i > 0 && sorted_nums[i] == sorted_nums[i - 1] {
                continue;
            }
            let mut l = i + 1;
            let mut r = n - 1;
            while l < r {
                let summ = sorted_nums[i] + sorted_nums[l] + sorted_nums[r];
                if summ == target {
                    return summ;
                }
                if (summ - target).abs() < (ans - target).abs() {
                    ans = summ;
                }
                match summ.cmp(&target) {
                    Ordering::Less => l += 1,
                    Ordering::Greater => r -= 1,
                    _ => {}
                }
            }
        }

        ans
    }
}

fn main() {
    let test_cases = vec![
        (vec![-1, 2, 1, -4], 1),
        (vec![0, 2, 1, -3], 1),
        (vec![-1, 2, 1, -4], 1),
        (vec![1, 1, 1, 0], -100),
        (vec![1, 1, 1, 0], 100),
        (vec![-1, 0, 1, 1, 55], 3),
    ];

    for (nums, target) in test_cases {
        let result = Solution::three_sum_closest(nums.to_vec(), target);
        println!(
            "The sum closest to {} for array {:?} is: {}",
            target, nums, result
        );
    }
}