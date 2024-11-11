struct Solution;

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mx = (1 << maximum_bit) - 1;
        let mut ans = Vec::with_capacity(nums.len());
        let mut xors = 0;

        for &num in &nums {
            xors ^= num;
            ans.push(xors ^ mx);
        }

        ans.reverse();
        ans
    }
}

fn main() {
    let nums = vec![0, 1, 2, 3, 4];
    let maximum_bit = 3;
    let result = Solution::get_maximum_xor(nums, maximum_bit);
    println!("{:?}", result); // Expected output: [7, 6, 5, 4, 3]
}
