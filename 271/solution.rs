impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut s = String::new();

        for word in strs {
            s.push_str(&format!("{}#{}", word.len(), word));
        }
        s
    }

    pub fn decode(s: String) -> Vec<String> {
        let mut strs = Vec::new();
        let mut idx = 0;
        let mut len_str = String::new();
        let bytes = s.as_bytes();

        while idx < s.len() {
            let c = bytes[idx] as char;
            if c == '#' {
                let len = len_str.parse::<usize>().unwrap();
                let word = &s[idx + 1..len + idx + 1];
                strs.push(word.into());
                idx = idx + len + 1;
                len_str = String::new();
            } else {
                len_str.push(c);
                idx += 1;
            }
        }
        strs
    }
}
