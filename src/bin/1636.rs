use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        for &num in &nums {
            *freq.entry(num).or_insert(0) += 1;
        }

        let mut nums: Vec<_> = nums.into_iter().collect();
        nums.sort_unstable_by(|a, b| {
            if freq[a] == freq[b] {
                b.cmp(a)
            } else {
                freq[a].cmp(&freq[b])
            }
        });

        nums
    }
}

fn main() {
    let nums = vec![1, 1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let result = Solution::frequency_sort(nums);
    println!("{:?}", result); // [4, 4, 4, 4, 3, 3, 3, 2, 2, 1, 1]
}

/* editorial code in c++, easy
class Solution {
public:
    vector<int> frequencySort(vector<int>& nums) {
        unordered_map<int, int> freq;
        for (int num : nums) {
            freq[num]++;
        }

        sort(nums.begin(), nums.end(), [&](int a, int b) {
            if (freq[a] == freq[b]) {
                return a > b;
            }
            return freq[a] < freq[b];
        });

        return nums;
    }
};

*/