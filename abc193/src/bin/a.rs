// use proconio::input;
// use super::proconio::input;
use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
    }

    let sa = a - b;
    let result: f64 = (sa * 100.0) / a;

    println!("{}", result);
}
