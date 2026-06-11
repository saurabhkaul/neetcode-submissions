impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut map:HashSet<i32> = HashSet::new();
        for n in nums{
            if map.contains(&n){
                return true
            }else{
                map.insert(n);
            }
        }
        return false;


    }
}
