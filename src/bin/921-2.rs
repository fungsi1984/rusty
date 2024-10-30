struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut open_brackets = 0;
        let mut min_adds_required = 0;

        for c in s.chars() {
            if c == '(' {
                open_brackets += 1;
            } else {
                // Match a closing bracket with an open one, or increment the counter if none exists
                if open_brackets > 0 {
                    open_brackets -= 1;
                } else {
                    min_adds_required += 1;
                }
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
