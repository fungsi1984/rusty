impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut connected_component_count = 0;
        let mut set_representatives = std::collections::HashMap::new();

        fn find_representative(
            element: i32,
            set_representatives: &mut std::collections::HashMap<i32, i32>,
            connected_component_count: &mut i32,
        ) -> i32 {
            if let Some(rep) = set_representatives.get(&element) {
                if *rep != element {
                    let new_rep =
                        find_representative(*rep, set_representatives, connected_component_count);
                    set_representatives.insert(element, new_rep);
                    new_rep
                } else {
                    element
                }
            } else {
                set_representatives.insert(element, element);
                *connected_component_count += 1;
                element
            }
        }

        fn merge_components(
            element_a: i32,
            element_b: i32,
            set_representatives: &mut std::collections::HashMap<i32, i32>,
            connected_component_count: &mut i32,
        ) {
            let rep_a =
                find_representative(element_a, set_representatives, connected_component_count);
            let rep_b =
                find_representative(element_b, set_representatives, connected_component_count);
            if rep_a != rep_b {
                set_representatives.insert(rep_b, rep_a);
                *connected_component_count -= 1;
            }
        }

        for stone in stones.iter() {
            merge_components(
                stone[0] + 1,
                stone[1] + 10002,
                &mut set_representatives,
                &mut connected_component_count,
            );
        }

        stones.len() as i32 - connected_component_count
    }
}

struct Solution;
fn main() {
    let stones = vec![
        vec![0, 0],
        vec![0, 1],
        vec![1, 0],
        vec![1, 2],
        vec![2, 1],
        vec![2, 2],
    ];
    println!("{}", Solution::remove_stones(stones));
}
