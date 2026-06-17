impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix_arr = vec![];
        let mut suffix_arr = vec![];
        let mut final_arr = vec![];
        let mut i = 0;
        while i<= nums.len()-1{
            let prefixes = {
                if i == 0 {
                    None

                }else{
                    nums.get(..i)
                }
            };
            match prefixes{
                None=>{
                    prefix_arr.push(1);
                },
                Some(slice)=>{
                    let x = {
                        if slice.len()>0 {
                            slice.iter().copied().reduce(|acc,x|acc * x).unwrap()
                        }else{
                            1
                        }
                    };
                    prefix_arr.push(x);
                }
            };
            let suffiexs = {
                if i == nums.len()-1{
                    None
                }else{
                    nums.get(i+1..)
                }
                
            };
            match suffiexs{
                None=>{
                    suffix_arr.push(1);
                },
                Some(slice)=>{
                    let x = slice.iter().copied().reduce(|acc,x|acc*x).unwrap();
                    println!("{x}");
                    suffix_arr.push(x)
                }
            };
            final_arr.push(prefix_arr.get(i).unwrap() * suffix_arr.get(i).unwrap());
            i+=1;
        }
        final_arr

    }
}
