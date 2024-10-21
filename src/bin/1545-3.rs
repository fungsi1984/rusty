struct Solution;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        match n {
            1 => '0', // Base case: S(1) is just "0"
            _ => {
                let mid_index = 1 << (n - 1);  // 2^(n - 1)
                
                match k {
                    _ if k == mid_index => '1', // Middle bit is always '1'
                    _ if k < mid_index => Self::find_kth_bit(n - 1, k), // Recurse on the first half
                    _ => { // Recurse on the second half and invert
                        let mirrored_bit = Self::find_kth_bit(n - 1, mid_index * 2 - k);
                        if mirrored_bit == '0' { '1' } else { '0' }
                    }
                }
            }
        }
    }
}

fn main() {
    let n = 3;
    let k = 5;
    let result = Solution::find_kth_bit(n, k);
    println!("The {}th bit in S({}) is: {}", k, n, result);
}
