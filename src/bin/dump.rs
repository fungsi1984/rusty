#[derive(Debug)]
struct ListNode {
    val: i32,
}

fn main() {
    let node = ListNode { val: 10 };
    let get_val = |n: ListNode| n.val; //closure
    let result = get_val(node);
    println!("result: {}", result); // Output: result: 10
}