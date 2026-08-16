use std::collections::HashSet;
use std::cmp::Ordering;
 
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        let mut result: HashSet<Vec<i32>> = HashSet::new();

        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // skip duplicate anchors
            }
            let mut left = i + 1;
            let mut right = n.wrapping_sub(1);
            while left < right && right < n {
                let sum = nums[i] + nums[left] + nums[right];
                match sum.cmp(&0) {
                    std::cmp::Ordering::Less => left += 1,
                    std::cmp::Ordering::Greater => right -= 1,
                    std::cmp::Ordering::Equal => {
                        result.insert(vec![nums[i], nums[left], nums[right]]);
                        left += 1;
                        right -= 1;
                    }
                }
            }
        }

        result.into_iter().collect()
    }
}
