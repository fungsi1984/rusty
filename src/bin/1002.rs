use core::cmp::min;

struct Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let words_size = words.len();
        let mut common_character_counts = vec![0; 26];
        let mut current_character_counts = vec![0; 26];
        let mut result = Vec::new();

        // Initialize commonCharacterCounts with the characters from the first
        // word
        for ch in words[0].chars() {
            common_character_counts[(ch as u8 - b'a') as usize] += 1;
        }

        for i in 1..words_size {
            current_character_counts.fill(0);

            // Count characters in the current word
            for ch in words[i].chars() {
                current_character_counts[(ch as u8 - b'a') as usize] += 1;
            }

            // Update the common character counts to keep the minimum counts
            for letter in 0..26 {
                common_character_counts[letter] = min(
                    common_character_counts[letter],
                    current_character_counts[letter],
                );
            }
        }

        // Collect the common characters based on the final counts
        for letter in 0..26 {
            for _ in 0..common_character_counts[letter] {
                let ch: char = (letter as u8 + b'a') as char;
                result.push(ch.to_string());
            }
        }

        result
    }
}

fn main() {
    let words = vec!["bella".to_string(), "label".to_string(), "roller".to_string()];
    let common_chars = Solution::common_chars(words);
    println!("{:?}", common_chars);
}