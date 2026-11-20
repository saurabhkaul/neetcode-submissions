use std::cmp::Ordering::{Greater,Less,Equal};

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let matrix = matrix.into_iter().flatten().collect::<Vec<i32>>();
        Self::binary_search(matrix,target)

    }
    fn binary_search(matrix:Vec<i32>,target:i32)-> bool{
        if matrix.is_empty(){
            false
        }else{
            let mid = 0_usize.midpoint(matrix.len());
            match matrix[mid].cmp(&target){
                Greater=>{
                    Self::binary_search(matrix[..mid].to_vec(),target)
                },
                Less=>{
                    Self::binary_search(matrix[mid+1..].to_vec(),target)
                },
                Equal=>{
                    true
                },
            }
        }
    }
}
