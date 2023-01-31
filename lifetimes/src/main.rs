// ---------- lifetimes of a String Slice ----------
// fn get_full_name() -> &'static str {
//     "Nitesh Tudu"
// }
//
// fn main() {
//     let full_name = get_full_name();
//     println!("Hello, {}!", full_name); // Hello, Nitesh Tudu!
// }

// ---------- lifetimes of generic types ----------
// fn get_large_name<'l>(a: &'l str, b: &'l str) -> &'l str {
//     // here the 'l is the lifetime of the return type
//     if a.len() > b.len() {
//         a
//     } else {
//         b
//     }
// }
//
// fn main() {
//     let result = get_large_name("Nitesh", "Yuu");
//     println!("winner is {}", result); // winner is Nitesh
// }

// ---------- lifetimes of structs ----------

// struct Person<'a> {
//     // here the 'a is the lifetime of the struct
//     // which means the struct will live as long as the instance of the struct
//     name: &'a str,
// }
//
// fn main() {}
//
// ---------- Lifetime elision ----------
// fn get_first_name(full_name: &str) -> &str {
//     // if we send a parameter of type &str, then the return type will be &str
//     // and they lifetime will be the same by the compiler
//     full_name
// }
//
// fn main() {}
//
// ---------- Lifetime impl ----------
//
struct Person<'a> {
    first_name: &'a str,
    last_name: &'a str,
}

impl<'a> Person<'a> {
    fn get_first_char(&self) -> &str {
        &self.first_name[0..1]
    }

    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

enum Animal<'a> {
    Dog { name: &'a str },
}

fn main() {}
