impl Solution {
    pub fn minimized_maximum(n: i32, quantities: Vec<i32>) -> i32 {
        let mut left = 1;
        let mut right = *quantities.iter().max().unwrap();

        while left < right {
            let mid = (left + right) / 2;
            let mut stores_needed = 0;

            for &quantity in &quantities {
                stores_needed += (quantity + mid - 1) / mid;
            }

            if stores_needed > n {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }
}

struct Solution;
fn main() {
    let n = 6;
    let quantities = vec![11, 6, 7];
    let result = Solution::minimized_maximum(n, quantities);
    println!("Minimized Maximum: {}", result);
}
