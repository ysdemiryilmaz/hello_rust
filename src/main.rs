use std::time::SystemTime;

fn main() {
    println!("Hello, Cargo!");
    let now = SystemTime::now();
    println!("{:#?}", now);
}
