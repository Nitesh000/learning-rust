use std::collections::HashMap;
pub fn hashmap() {
    let mut fruits: HashMap<&str, u8> = HashMap::new();
    fruits.insert("apple", 3);
    fruits.insert("banana", 5);
    fruits.insert("orange", 2);

    for (k, v) in &fruits {
        println!("I got {} {}", v, k);
    }
}
