use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    if 15 <= a + b && 8 <= b {
        // println!("x - ( a + 0.5) = {}", x - ( a + 0.5));
        // println!("a = {}", a);
        // アイスクリーム
        println!("{}", 1);
    } else if 10 <= a + b && 3 <= b {
        println!("{}", 2);
    } else if 3 <= a + b {
        println!("{}", 3);
    } else {
        println!("{}", 4);
    }
}
