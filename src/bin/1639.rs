struct Solution;

impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let word_length = words[0].len();
        let target_len = target.len();

        // Precompute the count of each character at each position in the words
        let mut counts = vec![vec![0; 26]; word_length];
        for (i, count_row) in counts.iter_mut().enumerate() {
            for word in &words {
                let c = word.chars().nth(i).unwrap() as usize - 'a' as usize;
                count_row[c] += 1;
            }
        }

        // Dynamic programming table: dp[i][j] = number of ways to form target[0..i] using words[0..j]
        let mut dp = vec![vec![0; word_length + 1]; target_len + 1];
        dp[0][0] = 1; // Base case: empty target and empty words

        // Fill the DP table
        for i in 0..=target_len {
            for j in 0..word_length {
                if dp[i][j] == 0 {
                    continue; // Skip invalid states
                }

                // Option 1: Skip the current character in words
                dp[i][j + 1] = (dp[i][j + 1] + dp[i][j]) % MOD;

                // Option 2: Use the current character in words (if it matches the target)
                if i < target_len {
                    let target_char = target.chars().nth(i).unwrap() as usize - 'a' as usize;
                    dp[i + 1][j + 1] = (dp[i + 1][j + 1] + dp[i][j] * counts[j][target_char] as i64) % MOD;
                }
            }
        }

        dp[target_len][word_length] as i32
    }
}

fn main() {
    let words = vec!["acca".to_string(), "bbbb".to_string(), "caca".to_string()];
    let target = "aba".to_string();
    let result = Solution::num_ways(words, target);
    println!("Number of ways: {}", result); // Output: Number of ways: 6
}