use std::ops::Deref;

struct BoxedValue<T> {
    value: T,
}

impl<T> BoxedValue<T> {
    fn new(value: T) -> Self {
        BoxedValue { value }
    }
}

impl<T> Deref for BoxedValue<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn print_integer(value: &i32) {
    println!("The value is {}", value);
}

fn main() {
    // let age: Box<u8> = Box::new(33);
    // let after_two_years = *age + 2;
    // println!(
    //     "My age is {} and after two years my age will be {}",
    //     age, after_two_years
    // ); // My age is 33 and after two years my age will be 35
    let age = BoxedValue::new(33); // BoxedValue<i32>
    println!("value is {}", *age); // value is 33
                                   // let actual_age: i32 = *age; // actualtype is i32
                                   // let ref_to_value = age.deref(); // ref_to_value is &i32
                                   // let other = *(age.deref()); // other is i32
    let value = BoxedValue::new(33);
    print_integer(&value);
}
