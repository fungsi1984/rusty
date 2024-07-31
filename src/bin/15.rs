impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums; // Take ownership of the input vector
        if nums.len() < 3 {
            return vec![];
        }

        nums.sort(); // Sort the vector in place

        let mut ans: Vec<Vec<i32>> = Vec::new();

        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // Skip duplicate elements
            }

            let mut l = i + 1;
            let mut r = nums.len() - 1;

            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if sum == 0 {
                    ans.push(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                    r -= 1;
                    while l < r && nums[l] == nums[l - 1] {
                        l += 1; // Skip duplicate elements
                    }
                    while l < r && nums[r] == nums[r + 1] {
                        r -= 1; // Skip duplicate elements
                    }
                } else if sum < 0 {
                    l += 1;
                } else {
                    r -= 1;
                }
            }
        }

        ans
    }
}

struct Solution;
fn main() {
    // Example usage
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let result = Solution::three_sum(nums);

    println!("Triplets that sum to zero are: {:?}", result);
}
