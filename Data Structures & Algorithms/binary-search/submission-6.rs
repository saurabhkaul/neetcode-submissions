use std::cmp::Ordering;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        //handling for 2 elems or less

        if nums.len() < 3{
            println!("in loop nums {nums:#?}");
            for (i,e) in nums.iter().enumerate(){
                if *e == target{
                     return i as i32;
                }
            }
            return -1;
        }

        let last_idx = nums.len() -1;
        let midpoint = 0_usize.midpoint(last_idx);
        if midpoint == 0  && nums[midpoint]!= target {
            return -1_i32
        }
        
        println!("midpoint = {midpoint}, nums = {nums:#?}");
        match nums[midpoint].cmp(&target){
            Ordering::Greater=>{
                Self::search(nums[..=midpoint].to_vec(),target)
                
            }
            Ordering::Equal =>{
                midpoint as i32
            },
            Ordering::Less =>{
                let m = Self::search(nums[midpoint..].to_vec(),target);
                if m == -1 {
                    m
                }else{
                    midpoint as i32 + m
               }
            }
        }
    }
    

}
