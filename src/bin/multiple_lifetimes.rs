#[derive(Debug)]
struct TravelPlan<'a, 'b>{
    from : &'a str,
    to: &'b str
}
fn main(){
    let from = String::from("Portland");
     let plan = {
        let to = String::from("bangor");
        let travel_plan = TravelPlan {
            from: &from,
            to: &to,
        };

        travel_plan.from
    };

    println!("{plan}");
}