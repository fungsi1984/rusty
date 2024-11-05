use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.is_empty() || words.is_empty() {
            return Vec::new();
        }

        let k = words.len();
        let n = words[0].len();
        let s_len = s.len();
        let mut ans = Vec::new();

        // Build frequency map for words
        let mut word_count = HashMap::new();
        for word in &words {
            *word_count.entry(word.clone()).or_insert(0) += 1;
        }

        // Only need to slide the window for each offset up to the word length
        for i in 0..n {
            let mut left = i;
            let mut right = i;
            let mut seen = HashMap::new();
            let mut count = 0;

            // Slide window of total length `k * n`
            while right + n <= s_len {
                let word = &s[right..right + n];
                right += n;

                if let Some(&freq) = word_count.get(word) {
                    *seen.entry(word.to_string()).or_insert(0) += 1;
                    count += 1;

                    // If word count exceeds the frequency in word_count, slide left window
                    while seen[word] > freq {
                        let left_word = &s[left..left + n];
                        *seen.get_mut(left_word).unwrap() -= 1;
                        left += n;
                        count -= 1;
                    }

                    // If current window has the correct number of words, record the starting index
                    if count == k {
                        ans.push(left as i32);
                    }
                } else {
                    // If the word is not in word_count, reset the window
                    seen.clear();
                    count = 0;
                    left = right;
                }
            }
        }

        ans
    }
}

fn main() {
    let s = "aaaaaaaaaaaa".to_string();
    let words = vec!["aaa".to_string(), "aaa".to_string(), "aaa".to_string()];
    let result = Solution::find_substring(s, words);
    println!("{:?}", result);
}
