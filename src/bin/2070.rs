struct Solution;

impl Solution {
    pub fn maximum_beauty(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; queries.len()];
        let mut items = items.clone();  // Make a mutable clone of items
        let mut max_beauty_so_far = vec![0; items.len() + 1];

        // Sort items by price (first element of each pair)
        items.sort_by_key(|item| item[0]);

        // Precompute max beauty for each prefix
        for i in 0..items.len() {
            max_beauty_so_far[i + 1] = max_beauty_so_far[i].max(items[i][1]);
        }

        // Compute answers for each query
        for (i, &query) in queries.iter().enumerate() {
            let index = Self::first_greater(&items, query);
            ans[i] = max_beauty_so_far[index];
        }

        ans
    }

    // Binary search to find the first item with a price greater than q
    fn first_greater(items: &[Vec<i32>], q: i32) -> usize {
        let mut l = 0;
        let mut r = items.len();
        while l < r {
            let m = (l + r) / 2;
            if items[m][0] > q {
                r = m;
            } else {
                l = m + 1;
            }
        }
        l
    }
}

fn main() {
    let items = vec![
        vec![1, 2], // price = 1, beauty = 2
        vec![3, 5], // price = 3, beauty = 5
        vec![6, 8], // price = 6, beauty = 8
    ];
    let queries = vec![2, 3, 5, 6]; // Queries for prices

    let result = Solution::maximum_beauty(items, queries);

    println!("{:?}", result); // Output: [2, 5, 5, 8]
}
