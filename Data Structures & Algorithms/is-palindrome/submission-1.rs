impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s:Vec<char> = s.to_lowercase().chars().filter(|c|c.is_alphanumeric()).collect();
        println!("{:?}",s);
        if s.is_empty(){
            true
        }
        else{
            let mut left = 0;
        let mut right = s.len()-1;
        let mut flag = true;
        while right > 0 {
            match s[left] == s[right]{
                true =>{
                    left+=1;
                    right-=1;
                    continue;
                },
                false=>{
                    flag = false;
                    break;
                }
            }
        }
        flag

        }
        
    }
}
