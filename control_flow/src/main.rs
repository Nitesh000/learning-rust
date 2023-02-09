// if expression
// fn main() {
//     let number = 3;
//
//     if number < 5 {
//         println!("{} is less that 5", number);
//     } else {
//         println!("{} is greater than 5", number);
//     } // 3 is less that 5
// }

// Multiple if else conditions
// fn main() {
//     let number = 6;
//
//     if number % 4 == 0 {
//         println!("{} is divisible by 4", number);
//     } else if number % 3 == 0 {
//         println!("{} is divisible by 3", number);
//     } else if number % 2 == 0 {
//         println!("{} is divisible by 2", number);
//     } else {
//         println!("{} is not divisible by 4, 3, or 2", number);
//     } // 6 is divisible by 2
// }

// using if in a let statement
// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//
//     println!("The value of number is: {}", number); // The value of number is: 5
// }

// Repetition with Loops
// loop
// fn main() {
//     // loop {
//     //     println!("again!");
//     // } // infinite loop
//
//     // returing a value from a loop
//     let mut count = 0;
//     let result = loop {
//         count += 1;
//
//         if count == 10 {
//             break count * 2; // break returns a value
//         }
//     };
//     println!("the result is {}", result); // the result is 20
// }

// loop labels to distinguish between loops
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;
//
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }
//
//         count += 1;
//     }
//     println!("End count = {count}");
// }

// Condition loops with while
// fn main() {
//     let mut number = 3;
//
//     while number != 0 {
//         println!("{}!", number);
//         number -= 1;
//     }
//     println!("LIFTOFF!!!");
// }

// Looping through a collection with for
fn main() {
    let a = [10, 20, 30, 40, 50];
    // for element in a {
    //     println!("the value is: {}", element);
    // }

    // another way
    // for element in a.iter() {
    //     println!("the value is: {}", element);
    // }

    // for loop with range
    // for number in 0..5 {  // here we can add functions like .rev() to reverse the range
    //     println!("{}. the value is: {}", number, a[number]);
    // }
    //
    // for loop with reverese range
    for number in (0..5).rev() {
        println!("{}. the value is: {}", number, a[number]);
    }
}
