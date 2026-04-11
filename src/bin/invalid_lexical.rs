fn main() {
    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];
    let favorite_citites = &cities[0..2];
    let plcaes = cities;
    // drop(cities);
    //println!("{favorite_citites:?}");

    // let some_cities = {
    //     let cities = vec![
    //     String::from("London"),
    //     String::from("New York"),
    //     String::from("Barcelona")
    // ];

    // &cities[0..2]
    // }

    let cities = vec![
        String::from("London"),
        String::from("new York"),
        String::from("Barcelona"),
    ];
    let a = select_first_two_elements(&cities);
    println!("{a:?}");

    {
        let coffees = [String::from("Latte"), String::from("Mocha")];
        let b = select_first_two_elements(&coffees);
        println!("{b:?}");
    }
}

fn select_first_two_elements(items: &[String]) -> &[String] {
    &items[..2]
}

// fn create_slice(items: Vec<i32>) -> &[i32] {
//     &items[0]
// }

// fn create() -> &i32 {
//     let age = 100;
//     &age
// }
