fn main() {
    let dog = String::from("Jimmy");
    {
        let my_pet = &dog;
        println!("{my_pet}");
    }

    println!("{dog}");

    {
        let my_pet = &dog;
        println!("{my_pet}");
    }
}
