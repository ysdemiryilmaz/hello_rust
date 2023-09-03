use std::time::SystemTime;

fn main() {
    println!("Hello, Cargo!");
    let now = SystemTime::now();
    println!("{:#?}", now);

    let x: i32 = 5;
    assert_eq!(x, 5); // if not equal, program will panic
    println!("Success!")
}
