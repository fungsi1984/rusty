impl Solution {
    // Check if we can place 'm' balls at 'position'
    // with each ball having at least 'x' gap.
    fn can_place_balls(x: i32, position: &Vec<i32>, m: i32) -> bool {
        // Place the first ball at the first position.
        let mut prev_ball_pos = position[0];
        let mut balls_placed = 1;

        // Iterate on each 'position' and place a ball there if we can place it.
        for &curr_pos in position.iter().skip(1) {
            // Check if we can place the ball at the current position.
            if curr_pos - prev_ball_pos >= x {
                balls_placed += 1;
                prev_ball_pos = curr_pos;
            }
            // If we've placed all 'm' balls, we can return early.
            if balls_placed == m {
                return true;
            }
        }
        // If all 'm' balls are placed, return 'true'.
        balls_placed == m
    }

    fn max_distance(position: Vec<i32>, m: i32) -> i32 {
        let mut answer = 0;
        let n = position.len();
        let mut position = position;

        // Sort the positions of balls.
        position.sort_unstable();

        // Initial search space.
        let mut low = 1;
        let mut high = (position[n - 1] as f64 / (m - 1) as f64).ceil() as i32;

        while low <= high {
            let mid = low + (high - low) / 2;
            // If we can place all balls having a gap at least 'mid',
            if Solution::can_place_balls(mid, &position, m) {
                // then 'mid' can be our answer,
                answer = mid;
                // and discard the left half search space.
                low = mid + 1;
            } else {
                // Discard the right half search space.
                high = mid - 1;
            }
        }

        answer
    }
}

struct Solution;

fn main() {
    // Example usage:
    let position = vec![1, 2, 3, 4, 7];
    let m = 3;
    let result = Solution::max_distance(position, m);
    println!("Maximum distance: {}", result); // Output: 3
}