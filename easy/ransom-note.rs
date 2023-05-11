use std::collections::HashMap;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut h: HashMap<char, i32> = HashMap::new();
        for c in magazine.chars() {
            let e = h.entry(c).or_insert(0);
            *e += 1;
        }
        for c in ransom_note.chars() {
            if !h.contains_key(&c) || h[&c] == 0 {
                return false;
            }
            *h.get_mut(&c).unwrap() -= 1;
        }
        true
    }
}
