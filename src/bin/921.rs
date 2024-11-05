struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut open_brackets = 0;
        let mut min_adds_required = 0;

        for c in s.chars() {
            match c {
                '(' => open_brackets += 1, // Increment open bracket count
                ')' if open_brackets > 0 => open_brackets -= 1, // Match with an open bracket
                ')' => min_adds_required += 1, // No open bracket, so increment min_adds_required
                _ => {}
            }
        }

        // Add the remaining unmatched open brackets
        min_adds_required + open_brackets
    }
}

fn main() {
    let test_string = "()))((".to_string();
    let result = Solution::min_add_to_make_valid(test_string);
    println!("Minimum additions to make the string valid: {}", result);
}
