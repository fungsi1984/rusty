impl Solution {
    pub fn maximum_swap(mut num: i32) -> i32 {
        let mut digits = Vec::new();
        let mut maxi = Vec::new();
        let mut i = 0;
        while num > 0 {
            let d = num % 10;
            digits.push(d);
            num /= 10;
            if d > digits[i] {
                i = digits.len() - 1;
            }
            maxi.push(i);
        }
        for i in (0..digits.len()).rev() {
            if digits[i] < digits[maxi[i]] {
                digits.swap(i, maxi[i]);
                break;
            }
        }
        digits.iter().rev().fold(0, |n, &d| n * 10 + d)
    }
}

struct Solution;
fn main() {
    // Example usage
    let num1 = 2736;
    let result1 = Solution::maximum_swap(num1);
    println!("Maximum swap of {} is {}", num1, result1); // Output: 7236

    let num2 = 9973;
    let result2 = Solution::maximum_swap(num2);
    println!("Maximum swap of {} is {}", num2, result2); // Output: 9973 (no swap)

    let num3 = 1993;
    let result3 = Solution::maximum_swap(num3);
    println!("Maximum swap of {} is {}", num3, result3); // Output: 9931

    let num4 = 222;
    let result4 = Solution::maximum_swap(num4);
      println!("Maximum swap of {} is {}", num4, result4); // Output: 222 (no swap)

    let num5 = 98368;
    let result5 = Solution::maximum_swap(num5);
    println!("Maximum swap of {} is {}", num5, result5); // Output: 98863
}