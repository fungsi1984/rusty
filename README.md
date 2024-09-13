
### how to run
```
cargo run --bin 1
```

### linker using lld
```
sudo dnf install clang, lld


- add config in .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
rustflags = ["-C", "linker=clang", "-C", "link-arg=-fuse-ld=lld"]

```

### linker using mold
```

sudo dnf install mold

- add config in .cargo/config.toml
[target.x86_64-unknown-linux-gnu]
linker = "/usr/bin/clang"
rustflags = ["-C", "link-arg=--ld-path=/usr/bin/mold"]

- check binary
readelf -p .comment target/debug/main
```




### notes
- 2, optimize for someone hate .unwrap()
    - 2-4, get rid of unwrap() with Some()
    - 2-5, use pattern matching like match with Some()
    - 2-7, using pointer dereference. pattern match and Some()
    - 2-8, the fastest one
    - 2-16, fast, short without unwrap
- 16, nice handling test case.
    - try not use clone(). and let see move works.
    - using to_vec could get rid clone()
- 94, using some "Option<Rc<RefCell<TreeNode>>>"
- 98,
    - had function from_vec for better input and output
    - 98-1, modified input for take null input
- 652, want see some troublesome "Vec<Option<Rc<RefCell<TreeNode>>>>"
- 1028,
    - 1028-2, the most hard i ever seen
- 1367,
    - 1367-2, painful linked list in binary tree in rust
- 1438-2, how to handle unwrap with some
- 1684
    - 1684-2, our first lifetime
    - 1684-3, if you hate to_string()
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
