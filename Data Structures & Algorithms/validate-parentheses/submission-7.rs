impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        let mut flag = true;
        for c in s.chars(){
            match c {
                '{' => stack.push('}'),
                '[' => stack.push(']'), 
                '(' => stack.push(')'),
                _ =>{
                    match c{
                        ')'|']'|'}' =>{
                            if let Some(peek) = stack.last(){
                                println!("{flag},{stack:#?},{c}");
                                if *peek != c{
                                    flag = false;
                                    break;
                                }else{
                                    stack.pop();
                                }
                            }else{
                                flag = false;
                                break;
                            }
                        },
                        _=>{}
                    }
                }
            }
        }
        flag && stack.is_empty()
    }
}
