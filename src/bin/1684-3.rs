use std::collections::HashSet;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut allowed_bits = 0;

        for c in allowed.chars() {
            allowed_bits |= 1 << (c as u8 - b'a');
        }

        let mut consistent_count = 0;

        for word in words {
            let mut is_consistent = true;

            for c in word.chars() {
                let bit = (allowed_bits >> (c as u8 - b'a')) & 1;

                if bit == 0 {
                    is_consistent = false;
                    break;
                }
            }

            if is_consistent {
                consistent_count += 1;
            }
        }

        consistent_count
    }

    pub fn count_consistent_strings_from_slice(allowed: &str, words: &[&str]) -> i32 {
        let allowed = allowed.to_string();
        let words = words.iter().map(|s| s.to_string()).collect();
        Self::count_consistent_strings(allowed, words)
    }
}

struct Solution;
// fn main() {
//     let allowed = "ab".to_string();
//     let words = vec![
//         "ad".to_string(),
//         "bd".to_string(),
//         "aaab".to_string(),
//         "baa".to_string(),
//         "badab".to_string(),
//     ];

//     let result = Solution::count_consistent_strings(allowed, words);
//     println!("Count of consistent strings: {}", result); // Output should be the count of consistent strings
// }

// if you hate to_string()

fn main() {
    let allowed = "cad";
    let words = &["cc", "acd", "b", "ba", "bac", "bad", "ac", "d"];

    let result = Solution::count_consistent_strings_from_slice(allowed, words);
    println!("Count of consistent strings: {}", result); // Output should be 4
}