mod hashmaps;
mod tuples;
mod vectors;

fn main() {
    let numbers: [u8; 5] = [1, 2, 3, 4, 5];
    let floats = [0.1f64, 0.2, 0.3, 0.4, 0.5];
    println!("numbers: {:?}", numbers);
    println!("floats: {:?}", floats);
    vectors::vecotrs();
    hashmaps::hashmap();
}
