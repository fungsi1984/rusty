use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let n = skill.len();
        let team_skill = skill.iter().sum::<i32>() / (n as i32 / 2);
        let mut ans: i64 = 0;
        let mut count: HashMap<i32, i32> = HashMap::new();

        // Count the frequency of each skill
        for &s in &skill {
            *count.entry(s).or_insert(0) += 1;
        }

        // Check if teams can be formed and compute the result
        for (&s, &freq) in &count {
            let required_skill = team_skill - s;
            if let Some(&required_freq) = count.get(&required_skill) {
                if required_freq != freq {
                    return -1;
                }
                ans += (s as i64) * (required_skill as i64) * (freq as i64);
            } else {
                return -1;
            }
        }

        ans / 2
    }
}

fn main() {
    // Example usage
    let skills = vec![1, 2, 3, 4, 5, 6];
    let result = Solution::divide_players(skills);
    println!("The result is: {}", result);
}
