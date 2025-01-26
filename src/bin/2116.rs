struct Solution;

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }

        let left_to_right_ok = Self::check(&s, &locked, true);
        let reversed_s: String = s.chars().rev().collect();
        let reversed_locked: String = locked.chars().rev().collect();
        let right_to_left_ok = Self::check(&reversed_s, &reversed_locked, false);

        left_to_right_ok && right_to_left_ok
    }

    fn check(s: &str, locked: &str, is_forward: bool) -> bool {
        let mut changeable = 0;
        let mut l = 0;
        let mut r = 0;

        for (c, lock) in s.chars().zip(locked.chars()) {
            match (c, lock) {
                (_, '0') => changeable += 1, // If the lock is '0', the character is changeable
                ('(', _) => l += 1,         // If the character is '(', increment l
                (')', _) => r += 1,         // If the character is ')', increment r
                _ => unreachable!(),        // This case should never happen
            }

            if is_forward && changeable + l - r < 0 {
                return false;
            }
            if !is_forward && changeable + r - l < 0 {
                return false;
            }
        }

        true
    }
}

fn main() {
    let s = String::from("()()");
    let locked = String::from("0000");
    println!("{}", Solution::can_be_valid(s, locked)); // Output: true
}
