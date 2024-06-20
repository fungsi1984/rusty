use std::collections::HashSet;

struct Solution;

impl Solution {
    fn longest_palindrome(s: String) -> i32 {
        let mut character_set: HashSet<char> = HashSet::new();
        let mut res = 0;

        // Loop over characters in the string
        for c in s.chars() {
            // If set contains the character, match found
            if character_set.contains(&c) {
                character_set.remove(&c);
                // add the two occurrences to our palindrome
                res += 2;
            } else {
                // add the character to the set
                character_set.insert(c);
            }
        }

        // if any character remains, we have at least one unmatched
        // character to make the center of an odd length palindrome.
        if !character_set.is_empty() {
            res += 1;
        }

        res
    }
}

#[test]
fn test_longest_palindrome() {
    let s = "abccccdd".to_string();
    let result = Solution::longest_palindrome(s);
    assert_eq!(result, 7);

    let s = "a".to_string();
    let result = Solution::longest_palindrome(s);
    assert_eq!(result, 1);

    let s = "ccc".to_string();
    let result = Solution::longest_palindrome(s);
    assert_eq!(result, 3);
}

fn main() {
    let s = "abccccdd".to_string();
    let result = Solution::longest_palindrome(s);
    println!("The length of the longest palindrome is: {}", result);
}
