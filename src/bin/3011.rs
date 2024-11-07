struct Solution;

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        // Variables to hold the previous set bit count, maximum of the previous segment,
        // maximum of the current segment, and minimum of the current segment.
        let mut prev_set_bits = 0;
        let mut prev_max = i32::MIN;
        let mut curr_max = i32::MIN;
        let mut curr_min = i32::MAX;

        for &num in &nums {
            // Calculate the number of set bits in the current number.
            let set_bits = num.count_ones();

            // Check if we are moving to a new segment.
            if set_bits != prev_set_bits {
                // Ensure that the previous segment's max is less than the current segment's min.
                if prev_max > curr_min {
                    return false;
                }
                // Start a new segment, updating previous and current max and min values.
                prev_set_bits = set_bits;
                prev_max = curr_max;
                curr_max = num;
                curr_min = num;
            } else {
                // Continue in the current segment, updating current max and min values.
                curr_max = curr_max.max(num);
                curr_min = curr_min.min(num);
            }
        }

        // Ensure that the final previous max is less than or equal to the final current min.
        prev_max <= curr_min
    }
}

fn main() {
    let nums = vec![3, 7, 8, 9, 6];
    let result = Solution::can_sort_array(nums);
    println!("Can sort array: {}", result);
}
