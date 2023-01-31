fn main() {
    let age: Option<i32> = None;
    let age1: Option<i32> = Some(10);
    let age_multiplied_by_2 = age.map(|age| age * 2);
    let age1_multiplied_by_2 = age1.map(|age| age * 2);
    println!("{}",age_multiplied_by_2.unwrap_or_default()); // prints 0
    println!("{}",age1_multiplied_by_2.unwrap_or_default()); // prints 20
}
