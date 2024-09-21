impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack = Vec::new();
        let mut filter = vec![true; s.len()];

        for (index, char) in s.chars().enumerate() {
            match char {
                '(' => stack.push(index),
                ')' => {
                    if stack.is_empty() {
                        filter[index] = false;
                    } else {
                        stack.pop();
                    }
                }
                _ => {}
            }
        }
        for idx in stack {
            filter[idx] = false;
        }
        s.chars()
            .enumerate()
            .filter_map(|(idx, ch)| if filter[idx] { Some(ch) } else { None })
            .collect()
    }
}
struct Solution;
fn main() {
    let s = String::from("lee(t(c)o)de)");
    let result = Solution::min_remove_to_make_valid(s);
    println!("{}", result); // Output: "lee(t(c)o)de"
}
