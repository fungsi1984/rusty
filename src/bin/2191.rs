struct Solution;

impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut store_pairs: Vec<(i32, usize)> = Vec::new();

        for (i, &num) in nums.iter().enumerate() {
            let mut mapped_value = 0;
            let mut temp = num;
            let mut place = 1;

            if temp == 0 {
                store_pairs.push((mapping[0], i));
                continue;
            }

            while temp != 0 {
                mapped_value = place * mapping[(temp % 10) as usize] + mapped_value;
                place *= 10;
                temp /= 10;
            }

            store_pairs.push((mapped_value, i));
        }

        store_pairs.sort_unstable();

        let mut answer: Vec<i32> = Vec::new();
        for (_, idx) in store_pairs {
            answer.push(nums[idx]);
        }

        answer
    }
}

fn main() {
    let mapping = vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6];
    let nums = vec![991, 338, 38];
    let result = Solution::sort_jumbled(mapping, nums);
    println!("{:?}", result); // prints: [338, 38, 991]
}