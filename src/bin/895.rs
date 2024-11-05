use std::collections::{HashMap, VecDeque};

struct FreqStack {
    max_freq: usize,
    count: HashMap<i32, usize>,
    count_to_stack: HashMap<usize, VecDeque<i32>>,
}

impl FreqStack {
    fn new() -> Self {
        FreqStack {
            max_freq: 0,
            count: HashMap::new(),
            count_to_stack: HashMap::new(),
        }
    }
    
    fn push(&mut self, val: i32) {
        // Update the frequency count for the value
        let freq = self.count.entry(val).or_insert(0);
        *freq += 1;

        // Update the stack of values with this frequency
        let stack = self.count_to_stack.entry(*freq).or_insert(VecDeque::new());
        stack.push_back(val);

        // Update max frequency if necessary
        self.max_freq = self.max_freq.max(*freq);
    }
    
    fn pop(&mut self) -> i32 {
        // Get the stack of elements with the current max frequency
        let stack = self.count_to_stack.get_mut(&self.max_freq).expect("No elements to pop");
        let val = stack.pop_back().expect("Stack should not be empty");

        // Decrease the frequency count for this value
        if let Some(freq) = self.count.get_mut(&val) {
            *freq -= 1;
        }

        // If no more elements have this frequency, decrease max_freq
        if stack.is_empty() {
            self.count_to_stack.remove(&self.max_freq);
            self.max_freq -= 1;
        }
        
        val
    }
}

fn main() {
    let mut obj = FreqStack::new();

    // Push elements onto the stack
    obj.push(5);
    obj.push(7);
    obj.push(5);
    obj.push(7);
    obj.push(4);
    obj.push(5);

    // Pop elements based on frequency
    println!("Popped: {}", obj.pop()); // Output: 5 (most frequent)
    println!("Popped: {}", obj.pop()); // Output: 7 (next most frequent)
    println!("Popped: {}", obj.pop()); // Output: 5 (next most frequent)
    println!("Popped: {}", obj.pop()); // Output: 4 (only element with freq 1)
}
