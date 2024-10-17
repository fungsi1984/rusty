impl Solution {
    fn maximum_swap(num: i32) -> i32 {
        let num_str = num.to_string();
        let n = num_str.len();
        let mut max_digit_index = -1;
        let mut swap_idx1 = -1;
        let mut swap_idx2 = -1;

        // Traverse the string from right to left, tracking the max digit and
        // potential swap
        for i in (0..n).rev() {
            let digit = num_str.chars().nth(i).unwrap();
            if max_digit_index == -1 || digit > num_str.chars().nth(max_digit_index as usize).unwrap() {
                max_digit_index = i as i32; // Update the index of the max digit
            } else if digit < num_str.chars().nth(max_digit_index as usize).unwrap() {
                swap_idx1 = i as i32; // Mark the smaller digit for swapping
                swap_idx2 = max_digit_index; // Mark the larger digit for swapping
            }
        }

        // Perform the swap if a valid swap is found
        let mut num_str_vec: Vec<char> = num_str.chars().collect();
        if swap_idx1 != -1 && swap_idx2 != -1 {
            num_str_vec.swap(swap_idx1 as usize, swap_idx2 as usize);
        }

        // Return the new number or the original if no swap occurred
        num_str_vec.iter().collect::<String>().parse::<i32>().unwrap()
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