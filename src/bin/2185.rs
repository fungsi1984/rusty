impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        // Helper function to perform the recursion
        fn helper(words: &[String], pref: &str, index: usize, count: i32) -> i32 {
            // Base case: if we've processed all words, return the count
            if index >= words.len() {
                return count;
            }

            // Check if the current word starts with the prefix
            let new_count = if words[index].starts_with(pref) {
                count + 1
            } else {
                count
            };

            // Recur for the next word in the vector
            helper(words, pref, index + 1, new_count)
        }

        // Start the recursion with index 0 and count 0
        helper(&words, &pref, 0, 0)
    }
}

struct Solution;
fn main() {
    // Convert Vec<&str> to Vec<String> in one go
    let words: Vec<String> = vec!["pay", "attention", "practice", "attend"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    let pref = "at".to_string();

    // Call the function
    let result = Solution::prefix_count(words, pref);
    println!("{}", result); // Output: 2
}