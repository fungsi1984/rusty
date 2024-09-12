use std::collections::HashSet;

// Define a utility function to create an iterator over borrowed words
fn stream_words<'a>(words: &'a [String]) -> impl Iterator<Item = &'a String> {
    words.iter()
}

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        // Convert allowed string to a HashSet of characters
        let allowed_chars: std::collections::HashSet<char> = allowed.chars().collect();

        // Use the utility function to stream words and count consistent strings
        stream_words(&words)
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
