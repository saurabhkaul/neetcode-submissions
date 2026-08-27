impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack:Vec<i32> = Vec::new();
        for strs in tokens.into_iter() {
            match strs.as_str() {
                "+" | "-" | "*" | "/" =>{
                  let num1 = stack.pop().unwrap();
                  let num2 = stack.pop().unwrap();
                  match strs.as_str() {
                    "+" => stack.push(num2 + num1),
                    "-" => stack.push(num2 - num1),
                    "*" => stack.push(num2 * num1),
                    "/" => stack.push(num2 / num1),
                    _ => unreachable!()
                  }

                },
                _ => {
                    println!("{strs}");
                    let num:i32 = strs.parse().unwrap();
                    stack.push(num)
                }

            }
        }
        stack.pop().unwrap()
    }
}


