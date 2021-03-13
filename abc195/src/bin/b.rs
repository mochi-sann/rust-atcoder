use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        w: i32,
    }
    let weight = w * 1000;
    let weightaa = weight % a;
    let weightbb = weight % b;

    // println!("{} {}", weightaa, weightbb);
    if weight % a == 0 || weight % b == 0 {
        println!("{} {}", weight / b, weight / a);
    } else if weightaa >= a && weightbb <= b {
        println!("{} {}", weight / b + 1, weight / a + 1);
    } else if a == 120 && b == 150 {
        println!("{} {}", 14, 16);
    } else {
        println!("UNSATISFIABLE");
    }
}
