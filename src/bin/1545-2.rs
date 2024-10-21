struct Solution;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        // Base case: when n is 1, the first string is just "0"
        if n == 1 {
            return '0';
        }
        
        // Calculate the middle index
        let mid_index = 1 << (n - 1);  // This is equivalent to 2^(n - 1)
        
        // If k is the middle index, return '1'
        if k == mid_index {
            return '1';
        }
        
        // If k is less than the middle index, recursively check the first half
        if k < mid_index {
            return Self::find_kth_bit(n - 1, k);
        }
        
        // Otherwise, check the mirrored bit in the second half
        let mirrored_bit = Self::find_kth_bit(n - 1, mid_index * 2 - k);
        return if mirrored_bit == '0' { '1' } else { '0' };
    }
}

fn main() {
    let n = 3;
    let k = 5;
    let result = Solution::find_kth_bit(n, k);
    println!("The {}th bit in S({}) is: {}", k, n, result);
}
