use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<char>, Vec<String>> = HashMap::new();

        for word in strs {
            let mut chars = word.chars().collect::<Vec<char>>();
            chars.sort();

            map.entry(chars).or_insert(vec![]).push(word);
        }

        map.into_values().collect()
    }
}
