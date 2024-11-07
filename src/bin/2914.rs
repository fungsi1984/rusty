struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut ans = 0;

        // Iterate over chunks of two characters
        for pair in s.chars().collect::<Vec<_>>().chunks(2) {
            if pair.len() == 2 && pair[0] != pair[1] {
                ans += 1;
            }
        }

        ans
    }
}

fn main() {
    let s = "abab".to_string();
    let result = Solution::min_changes(s);
    println!("Minimum changes required: {}", result);
}
