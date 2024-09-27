use std::collections::VecDeque;

struct MyCalendar {
    timeline: VecDeque<(i32, i32)>,
}

impl MyCalendar {
    fn new() -> Self {
        MyCalendar {
            timeline: VecDeque::new(),
        }
    }

    fn book(&mut self, start: i32, end: i32) -> bool {
        for &(s, e) in &self.timeline {
            if start.max(s) < end.min(e) {
                return false;
            }
        }
        self.timeline.push_back((start, end));
        true
    }
}

fn main() {
    let mut calendar = MyCalendar::new();

    println!("{}", calendar.book(10, 20)); // Should print: true
    println!("{}", calendar.book(15, 25)); // Should print: false
    println!("{}", calendar.book(20, 30)); // Should print: true
}
