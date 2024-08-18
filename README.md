
### how to run
```
cargo run --bin 1
```

### understanding this thing
- 2-4, optimize for someone hate .unwrap()
- 16, nice handling test case.
    - try not use clone(). and let see move works.
    - using to_vec could get rid clone()
- 94, using some "Option<Rc<RefCell<TreeNode>>>"
- 98,
    - had function from_vec for better input and output
    - 98-1, modified input for take null input
- 1438-2, how to handle unwrap with some
- 2058, nice idea for using Some()

### notes
- example folder is unlinked folder for mini program compile it with
    ```
        rustc factorial.rs
    ```



### textbook that i use
- Code Like a Pro in Rust, Brenden Matthews
