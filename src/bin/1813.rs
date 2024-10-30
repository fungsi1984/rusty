impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        if sentence1.len() == sentence2.len() {
            return sentence1 == sentence2;
        }

        let words1: Vec<&str> = sentence1.split_whitespace().collect();
        let words2: Vec<&str> = sentence2.split_whitespace().collect();
        let m = words1.len();
        let n = words2.len();

        if m > n {
            return Solution::are_sentences_similar(sentence2, sentence1);
        }

        let mut i = 0; // words1's index
        while i < m && words1[i] == words2[i] {
            i += 1;
        }
        while i < m && words1[i] == words2[i + n - m] {
            i += 1;
        }

        i == m
    }
}

struct Solution;
fn main() {
    let solution = Solution;
    let sentence1 = "I love programming".to_string();
    let sentence2 = "I love Rust programming".to_string();

    let result = Solution::are_sentences_similar(sentence1, sentence2);
    println!("Are sentences similar? {}", result);
}
