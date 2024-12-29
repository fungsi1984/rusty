impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let modulo = 1_000_000_007;
        let word_len = words[0].len();
        let target_len = target.len();

        let freq: Vec<[i32; 26]> = words
          .into_iter()
          .fold(vec![[0; 26]; word_len], |mut m, w| {
            w
              .as_bytes()
              .iter()
              .enumerate()
              .for_each(|(i, &c)| m[i][(c - 'a' as u8) as usize] += 1);
            m
          });
        //println!("{:?}", freq);
        let mut dp = vec![vec![0_i64; target_len + 1]; word_len + 1];
        (0..=word_len)
          .for_each(|i| dp[i][0] = 1);
        (1..=word_len)
          .flat_map(|i| (1..=target_len).map(move |j| (i, j)))
          .filter(|&(word_idx, _)| word_idx > 0)
          .for_each(|(word_idx, target_idx)| {
            if target_idx == 0 {
                dp[word_idx][target_idx] = 1;
            } else {
                dp[word_idx][target_idx] = dp[word_idx - 1][target_idx];
                let c = (target.as_bytes()[target_idx - 1] - 'a' as u8) as usize;
                dp[word_idx][target_idx] += (dp[word_idx - 1][target_idx - 1] * freq[word_idx - 1][c] as i64) % modulo;
                dp[word_idx][target_idx] %= modulo;
            }
          });
        dp[word_len][target_len] as i32
    }
}

struct Solution;
fn main() {
    let words = vec!["acca".to_string(), "bbbb".to_string(), "caca".to_string()];
    let target = "aba".to_string();
    let result = Solution::num_ways(words, target);
    println!("Number of ways: {}", result); // Output: Number of ways: 6
}