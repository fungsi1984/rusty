struct Solution;

impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        Solution::count_less(&nums, upper) - Solution::count_less(&nums, lower - 1)
    }

    fn count_less(nums: &Vec<i32>, sum: i32) -> i64 {
        let mut res = 0;
        let mut j = nums.len() - 1;
        
        for i in 0..nums.len() {
            while i < j && nums[i] + nums[j] > sum {
                j -= 1;
            }
            if i < j {
                res += (j - i) as i64;
            }
        }
        
        res
    }
}

fn main() {
    let nums = vec![0, 1, 7, 4, 4, 5];
    let lower = 3;
    let upper = 6;
    let result = Solution::count_fair_pairs(nums, lower, upper);
    println!("Output: {}", result); // Output: 6
}
