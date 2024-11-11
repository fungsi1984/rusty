struct Solution;

impl Solution {
    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        const MAX: usize = 1000;
        let primes = Self::sieve_eratosthenes(MAX);

        let mut prev_num = 0;
        for &num in &nums {
            // Find the largest prime `p` such that `p < num - prev_num`
            if let Some(&p) = primes.iter().take_while(|&&p| p < num - prev_num).last() {
                if num - p <= prev_num {
                    return false;
                }
                prev_num = num - p;
            } else {
                // If no such prime `p` exists, `num` itself must be greater than `prev_num`
                if num <= prev_num {
                    return false;
                }
                prev_num = num;
            }
        }

        true
    }

    fn sieve_eratosthenes(n: usize) -> Vec<i32> {
        let mut is_prime = vec![true; n];
        is_prime[0] = false;
        if n > 1 {
            is_prime[1] = false;
        }

        for i in 2..((n as f64).sqrt() as usize + 1) {
            if is_prime[i] {
                for j in (i * i..n).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }

        is_prime.iter()
            .enumerate()
            .filter_map(|(i, &prime)| if prime { Some(i as i32) } else { None })
            .collect()
    }
}

fn main() {
    let nums = vec![10, 6, 3];
    let result = Solution::prime_sub_operation(nums);
    println!("Result: {}", result); // Output whether the operation is successful
}
