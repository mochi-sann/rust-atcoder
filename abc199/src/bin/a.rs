use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }

    if (a.pow(2) + b.pow(2)) < c.pow(2) {
        println!("Yes");
    } else {
        println!("No");
    }
}
