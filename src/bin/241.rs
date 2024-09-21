use std::collections::HashMap;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut mem = HashMap::new();
        Solution::ways(&expression, &mut mem)
    }

    fn ways(s: &str, mem: &mut HashMap<String, Vec<i32>>) -> Vec<i32> {
        if let Some(result) = mem.get(s) {
            return result.clone();
        }

        let mut ans = Vec::new();

        for (i, ch) in s.chars().enumerate() {
            if ch == '+' || ch == '-' || ch == '*' {
                let left = Solution::ways(&s[0..i], mem);
                let right = Solution::ways(&s[i + 1..], mem);
                for &a in &left {
                    for &b in &right {
                        ans.push(match ch {
                            '+' => a + b,
                            '-' => a - b,
                            '*' => a * b,
                            _ => unreachable!(),
                        });
                    }
                }
            }
        }

        if ans.is_empty() {
            // ans.push(s.parse::<i32>().unwrap());
            if let Some(num) = s.parse::<i32>().ok() {
                ans.push(num);
            }
        }

        mem.insert(s.to_string(), ans.clone());
        ans
    }
}

struct Solution;
fn main() {
    let expression = "2*3-4*5".to_string();
    let result = Solution::diff_ways_to_compute(expression);
    println!("{:?}", result); // Output: [-34, -14, -10, -10, 10]
}
