impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let mut ans = vec![0; nums.len()]; // Initialize the result vector with zeros
        let mut i = 0; // Index for `ans`

        // First pass: add elements less than pivot
        for &num in &nums {
            if num < pivot {
                ans[i] = num;
                i += 1;
            }
        }

        // Second pass: add elements equal to pivot
        for &num in &nums {
            if num == pivot {
                ans[i] = num;
                i += 1;
            }
        }

        // Third pass: add elements greater than pivot
        for &num in &nums {
            if num > pivot {
                ans[i] = num;
                i += 1;
            }
        }

        ans // Return the result vector
    }
}

struct Solution;
fn main() {
    // Example usage
    let nums = vec![9, 12, 5, 10, 14, 3, 10];
    let pivot = 10;
    let result = Solution::pivot_array(nums, pivot);

    println!("{:?}", result); // Output: [9, 5, 3, 10, 10, 12, 14]
}