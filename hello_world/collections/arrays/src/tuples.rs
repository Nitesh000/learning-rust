pub fn create_tuple() {
    let num_and_str: (u8, &str) = (1, "one");
    // and we can also destruct the tuple
    let (num, str) = num_and_str;
    println!("num: {}", num);
    println!("str: {}", str);
}
