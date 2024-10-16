struct Solution;

impl Solution {
    // Helper function to find the two largest values among a, b, and c
    fn fun(a: i32, b: i32, c: i32) -> (i32, i32) {
        let k1 = a.max(b).max(c);  // Find the largest value
        let k2 = match k1 {
            _ if k1 == a => b.max(c),
            _ if k1 == b => a.max(c),
            _ => a.max(b),
        };
        (k1, k2)
    }

    // Main function to generate the longest diverse string
    pub fn longest_diverse_string(mut a: i32, mut b: i32, mut c: i32) -> String {
        let mut res = String::new();

        // Continue until there are no more characters to add
        while a + b + c > 0 {
            let (k1, _) = Solution::fun(a, b, c);
            let last_two: Vec<char> = res.chars().rev().take(2).collect();

            match (k1, last_two.as_slice()) {
                // If 'a' is the largest and last two are "aa"
                (k1, ['a', 'a']) if k1 == a => match (b, c) {
                    (b_val, _) if b_val > 0 => {
                        res.push('b');
                        b -= 1;
                    }
                    (_, c_val) if c_val > 0 => {
                        res.push('c');
                        c -= 1;
                    }
                    _ => return res,  // Stop if no available characters
                },
                // If 'b' is the largest and last two are "bb"
                (k1, ['b', 'b']) if k1 == b => match (a, c) {
                    (a_val, _) if a_val > 0 => {
                        res.push('a');
                        a -= 1;
                    }
                    (_, c_val) if c_val > 0 => {
                        res.push('c');
                        c -= 1;
                    }
                    _ => return res,  // Stop if no available characters
                },
                // If 'c' is the largest and last two are "cc"
                (k1, ['c', 'c']) if k1 == c => match (a, b) {
                    (a_val, _) if a_val > 0 => {
                        res.push('a');
                        a -= 1;
                    }
                    (_, b_val) if b_val > 0 => {
                        res.push('b');
                        b -= 1;
                    }
                    _ => return res,  // Stop if no available characters
                },
                // If no consecutive characters, just push the largest one
                (k1, _) => match k1 {
                    _ if k1 == a => {
                        res.push('a');
                        a -= 1;
                    }
                    _ if k1 == b => {
                        res.push('b');
                        b -= 1;
                    }
                    _ => {
                        res.push('c');
                        c -= 1;
                    }
                },
            }
        }

        res  // Return the final string
    }
}

fn main() {
    let result = Solution::longest_diverse_string(1, 1, 7);  // Example case
    println!("{}", result);
}
