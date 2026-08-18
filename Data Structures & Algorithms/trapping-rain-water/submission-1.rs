impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();

        if n < 3 {
            return 0;
        }

        let mut prefix_max = vec![0; n];
        let mut suffix_max = vec![0; n];

        prefix_max[0] = height[0];

        for i in 1..n {
            prefix_max[i] = prefix_max[i - 1].max(height[i]);
        }

        suffix_max[n - 1] = height[n - 1];

        for i in (0..n - 1).rev() {
            suffix_max[i] = suffix_max[i + 1].max(height[i]);
        }

        let mut vol = 0;

        for i in 0..n {
            let water_level = prefix_max[i].min(suffix_max[i]);
            vol += water_level - height[i];
        }

        vol
    }
}
