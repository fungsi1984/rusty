impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack: Vec<char> = Vec::new();

        for c in s.chars() {
            if c == 'B' && Solution::matches(&stack, 'A') {
                stack.pop();
            } else if c == 'D' && Solution::matches(&stack, 'C') {
                stack.pop();
            } else {
                stack.push(c);
            }
        }

        stack.len() as i32 // return the length as i32
    }

    fn matches(stack: &Vec<char>, c: char) -> bool {
        !stack.is_empty() && *stack.last().unwrap() == c
    }
}

struct Solution;
fn main() {
    let s = String::from("ABCDAB");
    let result = Solution::min_length(s);
    println!("Result: {}", result); // prints the result as i32
}
