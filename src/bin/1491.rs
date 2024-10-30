struct Solution;

impl Solution {
    pub fn can_arrange(arr: Vec<i32>, k: i32) -> bool {
        let mut count = vec![0; k as usize]; // equivalent to vector<int> count(k)

        // Populate the remainder counts
        for mut a in arr {
            a %= k;
            if a < 0 {
                a += k; // Ensure the remainder is non-negative
            }
            count[a as usize] += 1;
        }

        // Check if count[0] (numbers divisible by k) is even
        if count[0] % 2 != 0 {
            return false;
        }

        // Check the symmetry between count[i] and count[k-i]
        for i in 1..=(k / 2) {
            if count[i as usize] != count[(k - i) as usize] {
                return false;
            }
        }

        true
    }
}

fn main() {
    // Example usage
    let arr = vec![1, 2, 3, 4, 5, 10, 6, 7, 8, 9];
    let k = 5;
    let result = Solution::can_arrange(arr, k);
    println!("Result: {}", result); // Expected output: true

    // Another test case
    let arr2 = vec![1, 2, 3, 4, 5, 6];
    let k2 = 7;
    let result2 = Solution::can_arrange(arr2, k2);
    println!("Result: {}", result2); // Expected output: true
}
