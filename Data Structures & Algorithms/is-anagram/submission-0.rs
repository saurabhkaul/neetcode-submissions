impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut s_map: HashMap<char, u32> = HashMap::new();
        let mut t_map: HashMap<char, u32> = HashMap::new();

        for ch in s.chars() {
            *s_map.entry(ch).or_insert(0) += 1;
        }

        for ch in t.chars() {
            *t_map.entry(ch).or_insert(0) += 1;
        }

        s_map == t_map
    }
}
