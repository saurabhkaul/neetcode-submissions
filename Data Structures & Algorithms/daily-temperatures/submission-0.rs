impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut final_array = vec![0; temperatures.len()];
        let mut stack:Vec<usize> = vec![];
        for (idx, &temp) in temperatures.iter().enumerate(){
            while let Some(&top_idx) = stack.last(){
                if temperatures[top_idx] < temp{
                    stack.pop();
                    final_array[top_idx] = (idx - top_idx) as i32;
                }else{
                    break;
                }
            }
            stack.push(idx);
        }
        final_array
        
    }
}
