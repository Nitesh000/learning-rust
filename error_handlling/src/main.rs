// --------- Check ok and error with is_ok and is_err ---------
// fn get_user_name() -> Result<String, String> {
//     Err(())
// }
//
// fn get_another_user_name() -> Result<String, String> {
//     Ok(String::from("John"))
// }
//
// fn main() {
//     let user1 = get_user_name().is_ok();
//     let user2 = get_user_name().is_err();
//     let user3 = get_another_user_name().is_ok();
//     let user4 = get_another_user_name().is_err();
//     println!("User 1: {}, User2: {}", user1, user2); // User 1: false, User2: true
//     println!("User 3: {}, User4: {}", user3, user4); // User 3: true, User4: false
// }

// ----------- Early exit from Result errors ------------
fn get_first_name() -> Result<String, String> {
    Err("I don't know the first name".to_string())
}

fn get_last_name() -> Result<String, String> {
    Ok(String::from("Doe"))
}

fn get_full_name() -> Result<String, String> {
    let first_name = get_first_name()?; // the use of ? here is the same as using match
    let last_name = get_last_name()?; // ? will return the error if there is one
    Ok(format!("{} {}", first_name, last_name))
}

// fn main() {
//     let full_name = get_full_name();
//     let _error_length = full_name.map_err(|s| s.len()).unwrap_or_default();
//     println!("Error : {:?}", _error_length); // Full name: Err("I don't know the first name")
//     let first_name = get_first_name().unwrap_err();
//     println!("Error : {:?}", first_name); // Error : 25
// }

// main function returing Result
fn main() -> Result<(), String> {
    let full_name = get_full_name()?;
    println!("Full name: {:?}", full_name); // Full name: Err("I don't know the first name")
    Ok(())
}
