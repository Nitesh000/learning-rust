use std::fmt;

#[derive(Debug)]

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

trait HasFullName {
    fn full_name(&self) -> String;
}

trait HasAge {
    fn has_age(&self) -> u8;
}

trait PersonHasAge
where
    Self: HasAge,
{
    fn has_age(&self) -> u8;
}

impl<T> PersonHasAge for T
where
    T: HasAge,
{
    fn has_age(&self) -> u8 {
        self.has_age()
    }
}

impl HasAge for Person {
    fn has_age(&self) -> u8 {
        self.age
    }
}

impl HasFullName for Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

trait CanInitializeWithFullName {
    fn new(full_name: &str) -> Self;
}

impl CanInitializeWithFullName for Person {
    fn new(full_name: &str) -> Self {
        let parts: Vec<&str> = full_name.split(" ").collect();
        Person {
            first_name: parts[0].to_string(),
            last_name: parts[1].to_string(),
            age: 21,
        }
    }
}

fn print_full_name_and_age(value: &impl HasFullName) {
    println!("{}", value.full_name());
}

fn print_age(value: &impl HasAge) {
    println!("{}", value.has_age());
}

// fn print_details<T: HasFullName>(value: &T) {
//     println!("{}", value.full_name());
// }

fn print_details<T: HasFullName + CanRun>(value: &T) {
    // Multiple traits
    println!("{}", value.full_name());
    value.run();
}
// we can also write that above function with the where clause
// fn print_details<T>(value: &T)
// where
//     T: HasFullName + CanRun,
// {
//     // multiple traits
//     println!("{}", value.full_name());
//     value.run();
// }

trait CanRun {
    fn run(&self);
}

impl CanRun for Person {
    fn run(&self) {
        println!("{} is running", self.full_name());
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} is {} years old",
            self.first_name, self.last_name, self.age
        )
    }
}

fn main() {
    // let person = Person {
    //     first_name: "Nitesh".to_string(),
    //     last_name: "Tudu".to_string(),
    //     age: 21,
    // };
    // println!("{:?}", person); // Person { first_name: "Nitesh", last_name: "Tudu", age: 21 }
    let person = Person::new("Nitesh Tudu");
    // println!(
    //     "first name: {}, last name: {}, age: {}",
    //     person.first_name, person.last_name, person.age
    // ); // first name: Nitesh, last name: Tudu, age: 21
    // println!("{}", person); // Nitesh Tudu is 21 years old
    print_full_name_and_age(&person); // Nitesh Tudu
                                      // print_details(&person); // Nitesh Tudu
    print_details(&person); // Nitesh Tudu \nNitesh Tudu is running
    print_age(&person); // 21
}
