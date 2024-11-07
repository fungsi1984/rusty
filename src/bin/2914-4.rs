impl Solution {
    pub fn min_changes(s: String) -> i32 {
        s.as_bytes()
            .windows(2)
            .step_by(2)
            .filter(|window| window[0] != window[1])
            .count() as i32
    }
}

struct Solution;
fn main() {
    let s = "abab".to_string();
    let result = Solution::min_changes(s);
    println!("Minimum changes required: {}", result);
}
