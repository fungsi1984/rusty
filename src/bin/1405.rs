struct Solution;

impl Solution {
    // Helper function to find the two largest values among a, b, and c
    fn fun(a: i32, b: i32, c: i32) -> (i32, i32) {
        let k1 = a.max(b).max(c);  // Find the largest value
        let k2 = if k1 == a {
            b.max(c)
        } else if k1 == b {
            a.max(c)
        } else {
            a.max(b)
        };
        (k1, k2)
    }

    // Main function to generate the longest diverse string
    pub fn longest_diverse_string(mut a: i32, mut b: i32, mut c: i32) -> String {
        let mut res = String::new();

        // Continue until there are no more characters to add
        while a + b + c > 0 {
            let (k1, k2) = Solution::fun(a, b, c);

            // Check the last two characters of the result string
            let last_two: Vec<char> = res.chars().rev().take(2).collect();

            // If the largest character is 'a' and the last two characters are "aa"
            if k1 == a && last_two == ['a', 'a'] {
                if b > 0 {  // Use 'b' if available
                    res.push('b');
                    b -= 1;
                } else if c > 0 {  // Otherwise use 'c'
                    res.push('c');
                    c -= 1;
                } else {
                    return res;  // If neither are available, stop
                }
            }
            // If the largest character is 'b' and the last two characters are "bb"
            else if k1 == b && last_two == ['b', 'b'] {
                if a > 0 {  // Use 'a' if available
                    res.push('a');
                    a -= 1;
                } else if c > 0 {  // Otherwise use 'c'
                    res.push('c');
                    c -= 1;
                } else {
                    return res;  // If neither are available, stop
                }
            }
            // If the largest character is 'c' and the last two characters are "cc"
            else if k1 == c && last_two == ['c', 'c'] {
                if b > 0 {  // Use 'b' if available
                    res.push('b');
                    b -= 1;
                } else if a > 0 {  // Otherwise use 'a'
                    res.push('a');
                    a -= 1;
                } else {
                    return res;  // If neither are available, stop
                }
            }
            // Otherwise, add the largest character
            else if k1 == a {
                res.push('a');
                a -= 1;
            } else if k1 == b {
                res.push('b');
                b -= 1;
            } else {
                res.push('c');
                c -= 1;
            }
        }

        res  // Return the final string
    }
}

fn main() {
    let result = Solution::longest_diverse_string(1, 1, 7);  // Example case
    println!("{}", result);
}
