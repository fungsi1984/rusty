impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        match n {
            1 => '0', // Base case: S1 = "0"
            _ => {
                let mid_index = 2_i32.pow(n as u32 - 1); // Calculate the middle index for Sn (1-indexed)

                match k.cmp(&mid_index) {
                    std::cmp::Ordering::Equal => '1', // The middle element is always '1'
                    std::cmp::Ordering::Less => Solution::find_kth_bit(n - 1, k), // First half of the sequence
                    std::cmp::Ordering::Greater => {
                        let reflected_k = 2 * mid_index - k; // Reflect the index in the second half
                        match Solution::find_kth_bit(n - 1, reflected_k) {
                            '0' => '1',
                            '1' => '0',
                            _ => unreachable!(), // Should never happen
                        }
                    }
                }
            }
        }
    }
}

struct Solution;
fn main() {
    // Example usage
    println!("{}", Solution::find_kth_bit(3, 5)); // Output: '1'
}
