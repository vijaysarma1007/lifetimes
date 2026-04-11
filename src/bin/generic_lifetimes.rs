fn my_awesome_function(value: &i32) -> &i32 {
    value
}

fn select_first_two_elements<'a>(items: &'a [String]) -> &'a [String] {
    &items[..2]
}

fn main() {
    let cities = vec![
        String::from("London"),
        String::from("New York"),
        String::from("Barcelona"),
    ];

    let two_cities = {
        let citites_reference = &cities;
        select_first_two_elements(citites_reference)
    };

    println!("{two_cities:?}");
}
