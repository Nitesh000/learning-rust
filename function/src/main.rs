// fn main() {
//     println!("Hello, world!");
//     anoter_function();
// }
//
// fn anoter_function() {
//     println!("Another funtion.");
// }

// Parameters
// fn main() {
//     print_name_and_age(String::from("Nitesh"), 22);
// }
//
// fn print_name_and_age(name: String, age: u8) -> () {
//     println!("My name is {} and I am {} years old.", name, age);
// }

// statements and expressions
// fn main() {
//     let y = {
//         let x = 3;
//         x + 1 // this is the return value of the block
//     };
//     println!("The value of y is: {}", y); // 4
// }

// Function with Return values
fn five() -> i32 {
    5
}

fn main() {
    let x = five();
    println!("The value of x is: {}", x); // 5
}
