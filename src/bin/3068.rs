impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let k = k as i64;
        let mut res = 0;
        let mut sac = std::i64::MAX;
        let mut count = 0;
        for &i in &nums {
            let i = i as i64;
            let fi = i ^ k;
            if fi > i {
                res += fi;
                count += 1;
                sac = std::cmp::min(sac, fi - i);
            } else {
                res += i;
                sac = std::cmp::min(sac, i - fi);
            }
        }
        if count % 2 == 1 {
            res - sac
        } else {
            res
        }
    }
}

struct Solution;

fn main() {
    let nums = vec![4, 5, 6];
    let k = 2;
    let edges = vec![vec![1, 2], vec![2, 3]];

    let result = Solution::maximum_value_sum(nums, k, edges);
    println!("Maximum value sum: {}", result);
}
