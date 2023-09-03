use std::time::SystemTime;

fn main() {
    println!("Hello, Cargo!");
    let now = SystemTime::now();
    println!("{:#?}", now);

    let x: i32 = 5;
    let _y: i32; // uninitialized, but no warnings because of the underscore before variable name
    assert_eq!(x, 5); // if not equal, program will panic
    println!("Success!")
}
