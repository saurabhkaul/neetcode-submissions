impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut max_profit_arr = vec![];
    while left <= prices.len()-1{
        let mut max_profit = 0;

        if let Some(min) = prices[..left].iter().min(){
            if prices[left] > *min{
                max_profit = prices[left] - min;
            }
            
        }
        max_profit_arr.push(max_profit);
        left+=1;
    }

    *max_profit_arr.iter().max().unwrap()


    }
}
