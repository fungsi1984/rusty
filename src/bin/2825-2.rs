struct Solution;

impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        let mut i = 0; // str2's index
        let str2_chars: Vec<char> = str2.chars().collect();

        for c in str1.chars() {
            let next_char = Self::next_char(c);
            if c == str2_chars[i] || next_char == str2_chars[i] {
                i += 1;
                if i == str2_chars.len() {
                    return true;
                }
            }
        }

        false
    }

    fn next_char(c: char) -> char {
        match c {
            'a'..='y' => (c as u8 + 1) as char,
            'z' => 'a',
            _ => c,
        }
    }
}

fn main() {
    let str1 = String::from("abc");
    let str2 = String::from("ad");
    let result = Solution::can_make_subsequence(str1, str2);
    println!("{}", result); // Output: true
}
