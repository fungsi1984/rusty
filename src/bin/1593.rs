use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        let mut seen = HashSet::new();
        let mut max_count = 0;
        Solution::backtrack(&s, 0, &mut seen, 0, &mut max_count);
        max_count
    }

    fn backtrack(s: &str, start: usize, seen: &mut HashSet<String>, count: i32, max_count: &mut i32) {
        // Prune: If the current count plus remaining characters can't exceed maxCount, return
        if count + (s.len() - start) as i32 <= *max_count {
            return;
        }

        // Base case: If we reach the end of the string, update max_count
        if start == s.len() {
            *max_count = (*max_count).max(count);
            return;
        }

        // Try every possible substring starting from 'start'
        for end in start + 1..=s.len() {
            let substring = s[start..end].to_string();
            // If the substring is unique
            if !seen.contains(&substring) {
                // Add the substring to the seen set
                seen.insert(substring.clone());
                // Recursively count unique substrings from the next position
                Solution::backtrack(s, end, seen, count + 1, max_count);
                // Backtrack: remove the substring from the seen set
                seen.remove(&substring);
            }
        }
    }
}

fn main() {
    let s = "ababccc".to_string();
    let result = Solution::max_unique_split(s);
    println!("Max unique splits: {}", result);
}
