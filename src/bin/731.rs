use std::cmp::{max, min};

struct MyCalendarTwo {
    ranges: Vec<(i32, i32)>,
    overlaps: Vec<(i32, i32)>,
}

impl MyCalendarTwo {
    fn new() -> Self {
        MyCalendarTwo {
            ranges: Vec::new(),
            overlaps: Vec::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        // Check if the new event causes a triple booking
        for &(s, e) in &self.overlaps {
            if max(start, s) < min(end, e) {
                return false;
            }
        }

        // Add overlaps between the new event and existing events
        for &(s, e) in &self.ranges {
            let ss = max(start, s);
            let ee = min(end, e);
            if ss < ee {
                self.overlaps.push((ss, ee));
            }
        }

        // Add the new event to the ranges
        self.ranges.push((start, end));
        true
    }
}

fn main() {
    let mut calendar = MyCalendarTwo::new();

    // Bookings
    println!("{}", calendar.book(10, 20)); // returns true
    println!("{}", calendar.book(50, 60)); // returns true
    println!("{}", calendar.book(10, 40)); // returns true (creates an overlap with 10, 20)
    println!("{}", calendar.book(5, 15));  // returns false (triple overlap with 10-20 and 10-40)
    println!("{}", calendar.book(5, 10));  // returns true
    println!("{}", calendar.book(25, 55)); // returns true (creates an overlap with 50-60)
}
