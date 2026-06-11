impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut group:HashMap<[u32;26],Vec<String>> = HashMap::new();
        for s in strs {
            let arr = Self::freq_arr(&s);
            group.entry(arr).or_insert(Vec::new()).push(s);

        }
        group.into_values().collect()
    }
    pub fn freq_arr(s:&String) -> [u32;26]{
        let mut freq_arr = [0u32;26];
        for c in s.chars(){
            freq_arr[(c.to_ascii_lowercase() as u8 - b'a') as usize] += 1;
        }
        freq_arr

    }
}
