impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut map:HashMap<i32,u32> = HashMap::new();
        let mut result:Vec<i32> = vec![];
        for n in nums{
            *map.entry(n).or_insert(1) +=1;
        }
        for _ in 0..k {
            let max = map.clone().into_iter().max_by_key(|e|e.1);
            match max{
                Some((key,value))=>{
                    let (k,v) = map.remove_entry(&key).unwrap();
                    result.push(k);
                },
                None => {
                    break
                }
            }
        }
        result
        
    }
}
