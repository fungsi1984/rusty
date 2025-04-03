impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let mut ans = 0;
        let mut l = 0;
        let mut r = height.len() - 1;
        let mut max_l = height[l];
        let mut max_r = height[r];

        while l < r {
            if max_l < max_r {
                if max_l > height[l] {
                    ans += max_l - height[l];
                }
                l += 1;
                max_l = max_l.max(height[l]);
            } else {
                if max_r > height[r] {
                    ans += max_r - height[r];
                }
                r -= 1;
                max_r = max_r.max(height[r]);
            }
        }

        ans
    }
}

struct Solution;
fn main() {
    // Example 1:
    let height1 = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let result1 = Solution::trap(height1);
    println!("Example 1: Trapped water = {}", result1); // Output: 6

    // Example 2:
    let height2 = vec![4, 2, 0, 3, 2, 5];
    let result2 = Solution::trap(height2);
    println!("Example 2: Trapped water = {}", result2); // Output: 9

    // Example 3: Empty vector
    let height3: Vec<i32> = vec![];
    let result3 = Solution::trap(height3);
    println!("Example 3: Trapped water = {}", result3); // Output: 0

    // Example 4: Increasing then decreasing
    let height4 = vec![2, 0, 2];
    let result4 = Solution::trap(height4);
    println!("Example 4: Trapped water = {}", result4); // Output: 2

    // Example 5: Decreasing then increasing
    let height5 = vec![3, 2, 1, 2, 3];
    let result5 = Solution::trap(height5);
    println!("Example 5: Trapped water = {}", result5); // Output: 4

    // Example 6: All same height
    let height6 = vec![5, 5, 5, 5];
    let result6 = Solution::trap(height6);
    println!("Example 6: Trapped water = {}", result6); // Output: 0

    // Example 7: Only one element
    let height7 = vec![5];
    let result7 = Solution::trap(height7);
    println!("Example 7: Trapped water = {}", result7); // Output: 0
}
