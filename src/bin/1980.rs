struct Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums.len(); // Length of each string and number of strings
        let mut result = String::with_capacity(n);
        
        // Iterate over each index i and flip the bit at nums[i][i]
        for i in 0..n {
            match nums[i].chars().nth(i) {
                Some(ch) => {
                    if ch == '0' {
                        result.push('1');
                    } else {
                        result.push('0');
                    }
                }
                None => result.push('1'), // Unreachable in this problem due to constraints
            }
        }
        
        result
    }
}

// For local testing (not part of LeetCode submission)
fn main() {
    let nums = vec!["01".to_string(), "10".to_string()];
    let result = Solution::find_different_binary_string(nums);
    println!("{}", result); // Outputs "11"
    
    // Additional test case
    let nums2 = vec!["00".to_string(), "01".to_string()];
    let result2 = Solution::find_different_binary_string(nums2);
    println!("{}", result2); // Outputs "10"
}