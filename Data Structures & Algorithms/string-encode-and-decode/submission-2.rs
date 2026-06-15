impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut final_string = String::new();
        for s in strs{
            let len = s.len();
            final_string.push_str(&format!("{len}#{s}"))
        }
        final_string
    }




    pub fn decode(s: String) -> Vec<String> {
        let mut i:usize = 0;
        let mut len_buff = String::new();
        let mut final_arr = vec![];
        while i < s.len(){
            let current_char = s.chars().nth(i).unwrap();
            println!("char:{current_char},:buffer:{len_buff}");
            match current_char{
                '#'=>{
                    let len = len_buff.parse::<usize>().unwrap();
                    println!("before char:{current_char},:buffer:{len_buff},len:{len},i{i}");
                    len_buff.clear();
                    let next_str = s.get(i+1..i+1+len).unwrap();
                    i+=len+1; 
                    println!("after char:{current_char},:buffer:{len_buff},len:{len},next_str:{next_str},i{i}");
                    final_arr.push(next_str.to_string())
                }
                _=>{
                    len_buff.push(current_char);
                    i+=1
                }
            }
        }
        final_arr
    }
}