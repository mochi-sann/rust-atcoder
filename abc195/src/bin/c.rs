use proconio::input;
use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    input! {
        n: i32,
    }
    let mut result: i32 = 0;
    for i in 0..n {
        // println!("{}" ,i.to_string().chars().count());
        let mocgi = i.to_string().chars().count() / 3;
        result = result + mocgi.chars().count();
    }
    println!("{}", result);
}
