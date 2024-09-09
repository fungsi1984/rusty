
### how to run
```
cargo run --bin 1
```

### notes
- 2, optimize for someone hate .unwrap()
    - 2-4, get rid of unwrap() with Some()
    - 2-5, use pattern matching like match with Some()
    - 2-7, using pointer dereference. pattern match and Some()
    - 2-8, the fastest one
- 16, nice handling test case.
    - try not use clone(). and let see move works.
    - using to_vec could get rid clone()
- 94, using some "Option<Rc<RefCell<TreeNode>>>"
- 98,
    - had function from_vec for better input and output
    - 98-1, modified input for take null input
- 1367,
    - 1367-2, painful linked list in binary tree in rust
- 1438-2, how to handle unwrap with some
- 2058, nice idea for using Some()
- 2326, rust really wild in this questions, vector, linkedlist
    - 2326-4, our first unsafe
- 3217,
  - 3217-3, completely remove all unwrap

### compile
- example folder is unlinked folder for mini program compile it with
    ```
        rustc factorial.rs
    ```

### textbook that i use
- Code Like a Pro in Rust, Brenden Matthews
