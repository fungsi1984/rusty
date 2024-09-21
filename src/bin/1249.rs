impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack: Vec<usize> = Vec::new(); // unpaired '(' indices
        let mut chars: Vec<char> = s.chars().collect(); // Convert string to Vec<char> for mutability

        for i in 0..chars.len() {
            match chars[i] {
                '(' => stack.push(i), // Record the unpaired '(' index.
                ')' => {
                    if stack.is_empty() {
                        chars[i] = '#'; // Mark the unpaired ')' as '#'.
                    } else {
                        stack.pop(); // Find a pair!
                    }
                },
                _ => (), // Ignore other characters.
            }
        }

        // Mark the unpaired '(' as '#'.
        while let Some(idx) = stack.pop() {
            chars[idx] = '#';
        }

        // Collect and filter out all the '#' characters.
        chars.into_iter().filter(|&c| c != '#').collect()
    }
}

struct Solution;
fn main() {
    let s = String::from("lee(t(c)o)de)");
    let result = Solution::min_remove_to_make_valid(s);
    println!("{}", result); // Output: "lee(t(c)o)de"
}
