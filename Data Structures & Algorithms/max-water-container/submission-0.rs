use std::cmp::{min,max};


impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut left:i32 = 0;
        let mut right:i32 = (heights.len()-1).try_into().unwrap();
        let mut current_vol = |left:i32,right:i32|->i32{
            let breadth = right-left;
            let height = min(heights[left as usize],heights[right as usize]);
            breadth*height
        };
        let mut max_vol = 0;
        while left < right{
            max_vol = max(current_vol(left,right),max_vol);
            match heights[left as usize].cmp(&heights[right as usize]){
                Ordering::Greater=>right-=1,
                Ordering::Less=> left+=1,
                Ordering::Equal=>{
                    right-=1;
                    left+=1;
                }

            }

        }
        max_vol
        
    }
}
