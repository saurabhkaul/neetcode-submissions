impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars = vec![];
        for (p,s) in position.into_iter().zip(speed){
            cars.push((p,s))
        }

        //sort by position
        cars.sort_by(|a,b|b.0.cmp(&a.0));
        let mut fleet_stack = vec![];
        for car in cars {
            let time_to_target:f64 = (target - car.0) as f64/car.1 as f64;
            if let Some(topmost_time_to_target) = fleet_stack.last(){
                if time_to_target <= *topmost_time_to_target {
                    //do nothing, car joins the top fleet
                }else{
                    //car forms its own fleet
                    fleet_stack.push(time_to_target);
                }
            }else{
                fleet_stack.push(time_to_target)
            }
        }
        fleet_stack.len().try_into().unwrap()


    }
}
