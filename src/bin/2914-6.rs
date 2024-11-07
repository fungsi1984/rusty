impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut ans = 0;
        let mut i = 0;

        while i + 1 < s.len() {
            if s.as_bytes()[i] != s.as_bytes()[i + 1] {
                ans += 1;
            }
            i += 2;
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
