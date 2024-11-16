struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        s.len() == goal.len() && (s.clone() + &s).contains(&goal)
    }
}

fn main() {
    let s = "abcde".to_string();
    let goal = "cdeab".to_string();
    let result = Solution::rotate_string(s, goal.clone());
    println!("Is '{}' a rotation of '{}'? {}", goal, "abcde", result);

    let s2 = "abcde".to_string();
    let goal2 = "abced".to_string();
    let result2 = Solution::rotate_string(s2, goal2.clone());
    println!("Is '{}' a rotation of '{}'? {}", goal2, "abcde", result2);
}
