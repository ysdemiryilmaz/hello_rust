use std::time::SystemTime;

fn main() {
    println!("Hello, Cargo!");
    let now = SystemTime::now();
    println!("{:#?}", now);

    let x: i32 = 5;
    let _y: i32; // uninitialized, but no warnings because of the underscore before variable name
    assert_eq!(x, 5); // if not equal, program will panic
    println!("Success! 1");

    let mut a = 1; // mutable variable
    a += 2;

    assert_eq!(a, 3);
    println!("Success! 2");

    let k = 3;
    let l = 5;
    {
        println!("INNER SCOPE --> The value of k is {} and the value of l is {}", k, l);
    }
    println!("OUTER SCOPE --> The value of k is {} and the value of l is {}", k, l);
}
