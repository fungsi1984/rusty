struct Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut tmp = heights.clone();
        tmp.sort();
        let mut retval = 0;

        for i in 0..tmp.len() {
            if tmp[i] != heights[i] {
                retval += 1;
            }
        }

        retval
    }
}

fn main() {
    let heights = vec![1, 1, 4, 2, 1, 3];
    let operations_needed = Solution::height_checker(heights);
    println!("The minimum number of operations needed is: {}", operations_needed);
}
