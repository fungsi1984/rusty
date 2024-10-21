struct Solution;

impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        // Traverse through the expression
        for curr_char in expression.chars() {
            match curr_char {
                ',' | '(' => continue, // Skip commas and open parentheses
                't' | 'f' | '!' | '&' | '|' => {
                    stack.push(curr_char); // Push operators and boolean values to the stack
                }
                ')' => {
                    let mut has_true = false;
                    let mut has_false = false;

                    // Process the values inside the parentheses
                    while let Some(&top) = stack.last() {
                        match top {
                            '!' | '&' | '|' => break, // Stop when we encounter the operator
                            't' => has_true = true,
                            'f' => has_false = true,
                            _ => {}
                        }
                        stack.pop();
                    }

                    // Pop the operator and evaluate the subexpression
                    let op = stack.pop().unwrap();
                    match op {
                        '!' => stack.push(if has_true { 'f' } else { 't' }),
                        '&' => stack.push(if has_false { 'f' } else { 't' }),
                        '|' => stack.push(if has_true { 't' } else { 'f' }),
                        _ => unreachable!(),
                    }
                }
                _ => {}
            }
        }

        // The final result is at the top of the stack
        stack.pop().unwrap() == 't'
    }
}

fn main() {
    let expression = String::from("!(&(t,f))");
    let result = Solution::parse_bool_expr(expression);
    println!("{}", result);  // Output: true or false
}
