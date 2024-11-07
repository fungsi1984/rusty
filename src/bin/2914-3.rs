impl Solution {
    pub fn min_changes(s: String) -> i32 {
        (0..s.len() - 1)
            .step_by(2)
            .filter(|&i| s.as_bytes()[i] != s.as_bytes()[i + 1])
            .count() as i32
    }
}

struct Solution;
fn main() {
    let s = "abab".to_string();
    let result = Solution::min_changes(s);
    println!("Minimum changes required: {}", result);
}
