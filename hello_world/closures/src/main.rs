// closures are anonymous functions you can save in a variable or pass as arguments to other functions

fn main() {
    let doubler = |x| x * 2;
    let value = 4;
    let twice = doubler(value);
    println!("The doubler is : {}", twice); // 8

    let big_closure = |b, c| {
        let z = b + c;
        z * twice
    };

    let some_number = big_closure(1, 2);
    println!("some_number: {}", some_number); // 24
}
