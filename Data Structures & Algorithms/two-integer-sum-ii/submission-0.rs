use std::cmp::Ordering;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = numbers.len()-1;
        let mut result:Vec<i32> = vec![];
        

        while left < numbers.len()-1 || right > 0{
            let mut sum = numbers[left] + numbers[right];
            match sum.cmp(&target) {
                Ordering::Less => {
                    left+=1;
                },
                Ordering::Equal => {
                    result = vec![(left +1) as i32,(right+1) as i32];
                    break;
                },
                Ordering::Greater => {
                    right -=1;
                },
            }
        }
        result

    }
}
