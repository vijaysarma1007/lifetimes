fn say_hello() -> &'static str {
    "Hello"
}

fn main(){
     let greetings = say_hello();
     println!("greetings");
}