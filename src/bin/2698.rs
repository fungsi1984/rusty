struct Solution;

impl Solution {
    pub fn punishment_number(n: i32) -> i32 {
        let mut ans = 0;
        for i in 1..=n {
            if Self::is_possible(0, 0, &(i * i).to_string(), 0, i) {
                ans += i * i;
            }
        }
        ans
    }

    // Returns true if the sum of any split of `num_chars` equals the target.
    fn is_possible(accumulate: i32, running: i32, num_chars: &str, s: usize, target: i32) -> bool {
        if s == num_chars.len() {
            return target == accumulate + running;
        }
        let d = num_chars.chars().nth(s).unwrap().to_digit(10).unwrap() as i32;
        Self::is_possible(accumulate, running * 10 + d, num_chars, s + 1, target)
            || Self::is_possible(accumulate + running, d, num_chars, s + 1, target)
    }
}

fn main() {
    let n = 10;
    println!("Punishment number for {}: {}", n, Solution::punishment_number(n));
}