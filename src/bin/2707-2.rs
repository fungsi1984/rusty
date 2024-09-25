impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut res = vec![0; s.len() + 1];

        for i in (0..s.len()).rev() {
            res[i] = res[i + 1] + 1;
            for w in &dictionary {
                let w_len = w.len();
                if i + w_len <= s.len() && &s[i..i + w_len] == w {
                    res[i] = res[i].min(res[i + w_len]);
                }
            }
        }

        return res[0];
    }
}

struct Solution;
fn main() {
    let s = String::from("applepie");
    let dictionary = vec![String::from("apple"), String::from("pie")];

    let result = Solution::min_extra_char(s, dictionary);
    println!("Minimum extra characters: {}", result);
}
