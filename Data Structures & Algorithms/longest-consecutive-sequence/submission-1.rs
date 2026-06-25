impl Solution {
    pub fn longest_consecutive(nums:Vec<i32>) -> i32 {
        if nums.len() == 0{
            return 0;
        };
        let mut nums = nums;
        nums.sort();
        let mut i = 0;
        let mut sq_count = 0;
        let mut all_sq = vec![];
        while i <= nums.len()-1{
            sq_count +=1;
            loop {
                let cond = {
                    match (nums.get(i), nums.get(i + 1)) {
                        (Some(a), Some(b)) => Some(a - b),
                        _ => None,
                        }
                };
                match cond{
                    Some(0) => {
                        i+=1;
                    }
                    Some(-1) =>{
                        sq_count+=1;
                        i+=1;
                    }
                    _=>{
                        all_sq.push(sq_count);
                        sq_count = 0;
                        i+=1;
                        break;
                    }
                }
            };
        }
        match all_sq.iter().max(){
                Some(n) => *n as i32,
                None => 0
            }
    }
}