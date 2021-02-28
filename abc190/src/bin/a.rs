// use proconio::input;
// use super::proconio::input;
use proconio::input;

fn main() {
    input! {
        a: i32,
        b: f64,//価格
    }
    let mut result: f64 = 0.0;
    for i in 0..n {
        input! {
            a: f64,//距離
            p: f64,//価格
            x: f64,//在庫
        };

        if x - (a + 0.5) > 0.0 && (result > p || result == 0.0) {
            // println!("x - ( a + 0.5) = {}", x - ( a + 0.5));
            // println!("a = {}", a);
            result = p;
        }
    }
    println!("{}", result);
}
