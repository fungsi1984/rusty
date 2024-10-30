struct Solution;

impl Solution {
    pub fn remove_subfolders(mut folder: Vec<String>) -> Vec<String> {
        // Sort the folder list
        folder.sort();

        let mut ans = Vec::new();
        let mut prev = String::new();

        for f in folder {
            match (prev.is_empty(), f.starts_with(&prev), f.get(prev.len()..)) {
                // Case where the current folder is a subfolder of the previous one
                (false, true, Some(suffix)) if suffix.starts_with('/') => {
                    continue;
                }
                // Default case: folder is not a subfolder, so add it
                _ => {
                    ans.push(f.clone());
                    prev = f;
                }
            }
        }

        ans
    }
}

fn main() {
    let mut folder = vec![
        String::from("/a"),
        String::from("/a/b"),
        String::from("/c/d"),
        String::from("/c/d/e"),
        String::from("/c/f"),
    ];

    let result = Solution::remove_subfolders(folder);

    for f in result {
        println!("{}", f);
    }
}
