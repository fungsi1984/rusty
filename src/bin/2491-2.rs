use std::collections::HashMap;


impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let n: i64 = skill.len() as i64; // Convert usize to i64 here

        let team_skill = skill.iter().map(|&s| s as i64).sum::<i64>() / (n / 2);
        let mut ans: i64 = 0;
        let mut count: HashMap<i64, i64> = HashMap::new(); // Change to i64

        // Count the frequency of each skill
        for &s in &skill {
            *count.entry(s as i64).or_insert(0) += 1; // Cast to i64
        }

        // Check if teams can be formed and compute the result
        for (&s, &freq) in &count {
            let required_skill = team_skill - s;
            if let Some(&required_freq) = count.get(&required_skill) {
                if required_freq != freq {
                    return -1;
                }

                // Directly use i64 values
                ans += s * required_skill * freq;
            } else {
                return -1;
            }
        }

        ans / 2
    }
}


struct Solution;
fn main() {
    // Example usage
    let skills = vec![1, 2, 3, 4, 5, 6];
    let result = Solution::divide_players(skills);
    println!("The result is: {}", result);
}
