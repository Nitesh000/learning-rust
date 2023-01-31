fn get_full_name() -> &str {
    // error: `get_full_name` does not live long enough
    "Nitesh Tudu"
}

fn main() {
    let full_name = get_full_name();
    println!("Hello, {}!", full_name); // error: borrowed value does not live long enough
}
