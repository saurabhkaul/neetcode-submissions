impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut result = vec![];
        for i in 0..nums.len(){
            for j in i+1..nums.len(){
                let first = nums.get(i).unwrap();
                let second = nums.get(j).unwrap();
                match first + second == target{
                    true => {
                        result = vec![(i as i32).clone(),(j as i32).clone()];
                        return result;
                        },
                    false => {
                        continue
                    }
                }
            }
        }
        result

    }
}