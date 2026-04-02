use from_input::FromInput;

fn main() {
    let name = String::from_input("Enter your name >> ").unwrap();
    println!("Hello, {}!", name);
}
