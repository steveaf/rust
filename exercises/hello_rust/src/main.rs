const MAX_USERS: u32 = 100;

// fn main() {
//     let mut x: i32 = 5;
//     println!("INITIAL value of x is: {}", x);
//     x = 6;
//     println!("CHANGED x is: {}", x);
// }


fn main() {
    let age: i32 = 30;
    let price: f32 = 10.99;
    let is_active: bool = true;
    let initial: char = 'A';

    let count: u64 =1_000_000;
    let pi: f32 = 3.14159;

    // change then type! 
    let value: &str = "4";
    let value: i32 = value.parse().unwrap();
    println!("The value is: {}", value);
}
