struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        s.as_bytes()
            .chunks(2)
            .filter(|chunk| chunk.len() == 2 && chunk[0] != chunk[1])
            .count() as i32
    }
}

fn main() {
    let s = "abab".to_string();
    let result = Solution::min_changes(s);
    println!("Minimum changes required: {}", result);
}
