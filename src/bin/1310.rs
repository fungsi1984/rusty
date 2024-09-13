struct Solution;

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix_xor = Vec::with_capacity(arr.len());
        let mut current_xor = 0;

        // Step 1: Create the prefix XOR array
        for &num in arr.iter() {
            current_xor ^= num;
            prefix_xor.push(current_xor);
        }

        // Step 2: Resolve each query using the prefix XOR array
        let mut result = Vec::with_capacity(queries.len());
        for q in queries {
            let start = q[0] as usize;
            let end = q[1] as usize;
            if start > 0 {
                result.push(prefix_xor[end] ^ prefix_xor[start - 1]);
            } else {
                result.push(prefix_xor[end]);
            }
        }

        result
    }
}

fn main() {
    let arr = vec![1, 3, 4, 8];
    let queries = vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]];
    let result = Solution::xor_queries(arr, queries);
    println!("{:?}", result); // Output: [2, 7, 14, 8]
}
