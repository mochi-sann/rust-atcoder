use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    if n == 0 {
        println!("0");
        return;
    }
    let mut amin: i32 = 1;
    let mut bbig: i32 = 10000;
    // Aの処理
    for _i in 0..n  {
        input! {
            a: i32,

        }
        // println!("aaaaaa {}", a);

        if amin < a {
            amin = a;
        }
    }
    // Bの処理
    for _i in 0..n  {
        input! {

            b: i32,
        }
        // println!("bbbbbb {}", b);

        if bbig > b {
            bbig = b;
        }
    }
    // println!("anow {}  bnow {}", amin, bbig);

    if bbig - amin >= 0 {
        println!("{}", bbig - amin + 1);
    } else {
        println!("0");
    }
}
