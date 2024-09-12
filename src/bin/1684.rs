use std::collections::HashSet;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        // Convert allowed string to a HashSet of characters
        let allowed_chars: std::collections::HashSet<char> = allowed.chars().collect();

        // Filter words and count the consistent strings
        words
            .iter()
            .filter(|word| word.chars().all(|c| allowed_chars.contains(&c)))
            .count() as i32
    }
}

struct Solution;
fn main() {
    let allowed = "ab".to_string();
    let words = vec![
        "ad".to_string(),
        "bd".to_string(),
        "aaab".to_string(),
        "baa".to_string(),
        "badab".to_string(),
    ];

    let result = Solution::count_consistent_strings(allowed, words);
    println!("Count of consistent strings: {}", result); // Output should be the count of consistent strings
}
