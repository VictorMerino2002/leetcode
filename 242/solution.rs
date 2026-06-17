use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_map = HashMap::<char, u32>::new();
        let mut t_map = HashMap::<char, u32>::new();

        let mut s_chars = s.chars();
        let mut t_chars = t.chars();
        for _ in 0..s.len() {
            let s_char = s_chars.next().unwrap();
            *s_map.entry(s_char).or_insert(0) += 1;

            let t_char = t_chars.next().unwrap();
            *t_map.entry(t_char).or_insert(0) += 1;
        }

        s_map == t_map
    }
}
