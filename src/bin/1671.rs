use std::cmp::Ordering;
use std::cmp::min;

impl Solution {
    fn get_longest_increasing_subsequence_length(v: &Vec<i32>) -> Vec<usize> {
        let mut lis_len = vec![1; v.len()];
        let mut lis = vec![v[0]];

        for i in 1..v.len() {
            let index = lis
                .binary_search_by(|&x| if x < v[i] { Ordering::Less } else { Ordering::Greater })
                .unwrap_or_else(|x| x);

            if index == lis.len() {
                lis.push(v[i]);
            } else {
                lis[index] = v[i];
            }

            lis_len[i] = lis.len();
        }

        lis_len
    }

    fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let lis_length = Self::get_longest_increasing_subsequence_length(&nums);

        let mut rev_nums = nums.clone();
        rev_nums.reverse();
        let mut lds_length = Self::get_longest_increasing_subsequence_length(&rev_nums);
        lds_length.reverse();

        let mut min_removals = std::i32::MAX;
        for i in 1..n - 1 {
            if lis_length[i] > 1 && lds_length[i] > 1 {
                min_removals = min(
                    min_removals,
                    (n as i32 - lis_length[i] as i32 - lds_length[i] as i32 + 1),
                );
            }
        }

        min_removals
    }
}

struct Solution;
fn main() {
    let nums = vec![2, 1, 1, 5, 6, 2, 3, 1];
    let result = Solution::minimum_mountain_removals(nums);
    println!("Minimum mountain removals: {}", result);
}
