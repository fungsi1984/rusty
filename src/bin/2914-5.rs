impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut ans = 0;
        for i in (0..s.len() - 1).step_by(2) {
            if s.as_bytes()[i] != s.as_bytes()[i + 1] {
                ans += 1;
            }
        }
        ans
    }
}

struct Solution;
fn main() {
    let s = "abab".to_string();
    let result = Solution::min_changes(s);
    println!("Minimum changes required: {}", result);
}
