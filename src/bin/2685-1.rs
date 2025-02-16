struct Solution;

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        let mut ans = 0;
        for i in 1..=n {
            if Self::is_possible(0, 0, &(i * i).to_string(), 0, i) {
                ans += i * i;
            }
        }
        ans
    }

    // Returns true if the sum of any split of `num_chars` equals the target.
    fn is_possible(accumulate: i32, running: i32, num_chars: &str, s: usize, target: i32) -> bool {
        if s == num_chars.len() {
            return target == accumulate + running;
        }

        // Safely handle character access and digit conversion
        let d = if let Some(c) = num_chars.chars().nth(s) {
            if let Some(digit) = c.to_digit(10) {
                digit as i32
            } else {
                // If the character is not a valid digit, return false
                return false;
            }
        } else {
            // If `s` is out of bounds, return false
            return false;
        };

        // Recursively check both possibilities:
        // 1. Continue forming the current number
        // 2. Split and add the current running number to the accumulated sum
        Self::is_possible(accumulate, running * 10 + d, num_chars, s + 1, target)
            || Self::is_possible(accumulate + running, d, num_chars, s + 1, target)
    }
}

fn main() {
    let n = 10;
    let result = Solution::punishment_number(n);
    println!("Punishment number for {}: {}", n, result); // Output: Punishment number for 10: 182
}
