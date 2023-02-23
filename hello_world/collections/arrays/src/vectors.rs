pub fn vecotrs() {
    let mut numbers_vec: Vec<u8> = Vec::new(); // create an empty vector
    numbers_vec.push(1); // add an element to the vector
    numbers_vec.push(2); // add another element to the vector

    let mut vec_with_macro = vec![1, 2, 3]; // create a vector with the vec! macro
    let _ = vec_with_macro.pop(); // remove the last element from the vector

    let message = if numbers_vec == vec_with_macro {
        "They are equal"
    } else {
        "They are not equal"
    };
    println!("{}", message);
}
