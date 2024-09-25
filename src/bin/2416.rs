struct TrieNode {
    next: [Option<Box<TrieNode>>; 26],
    cnt: i32,
}

impl TrieNode {
    // Create a new TrieNode.
    fn new() -> Self {
        TrieNode {
            next: Default::default(),
            cnt: 0,
        }
    }

    // Insert a word into the trie.
    fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            let index = (c as usize) - ('a' as usize);
            if node.next[index].is_none() {
                node.next[index] = Some(Box::new(TrieNode::new()));
            }
            node = node.next[index].as_mut().unwrap();
            node.cnt += 1;
        }
    }

    // Calculate the prefix count for a given word.
    fn count(&self, s: &str) -> i32 {
        let mut node = self;
        let mut ans = 0;
        for c in s.chars() {
            let index = (c as usize) - ('a' as usize);
            if let Some(ref next_node) = node.next[index] {
                ans += next_node.cnt;
                node = next_node;
            } else {
                break;
            }
        }
        ans
    }
}

impl Solution {
    // Calculate the prefix scores for each word in the list.
    fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let mut root = TrieNode::new();
        for word in &words {
            root.insert(word);
        }
        words.iter().map(|word| root.count(word)).collect()
    }
}

struct Solution;
fn main() {
    let words = vec!["abc".to_string(), "ab".to_string(), "bc".to_string(), "b".to_string()];
    let result = Solution::sum_prefix_scores(words);
    println!("{:?}", result);
}
