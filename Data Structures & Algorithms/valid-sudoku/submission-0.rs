impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut flag = true;
        let mut i = 0;
        let mut j = 0;

        let checkpoints = [0,3,6];
        

        while i<9{
            let row_check = Self::freq_check(&board[i]);
            if row_check == false{
                flag = false;
                break;
            }
            while j<9{
                //check the i th row and jth col
                let col_check = Self::freq_check(&board.iter().map(|r|r[j]).collect());
                if col_check == false{
                    flag = false;
                    break;
                }
                if checkpoints.contains(&i) && checkpoints.contains(&j){
                    //we are in a sub grid
                    let mut sub_grid = vec![];
                    for i in i..i+3{
                        for j in j..j+3{
                            sub_grid.push(board[i][j]);
                        }
                    }
                    let sub_grid_check = Self::freq_check(&sub_grid);
                    if sub_grid_check == false{
                        flag = false;
                        break;
                    }
                }
                j+=1;
            }
            i+=1;
        }
        flag

    }

    
    pub fn freq_check(arr:&Vec<char>)->bool{
        let mut freq_arr = [0u8;10];
        for c in arr{
            match c{
                '.'=>continue,
                 _=>{
                    let num = c.to_digit(10).unwrap() as usize;
                    freq_arr[num]+=1;
                }
            }
        }
        match freq_arr.iter().max(){
            None => false,
            Some(n)=> *n <= 1,
        }
    }
}
