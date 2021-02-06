use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        v: i64,
        t: i64,
        s: i64,
        d: i64,
    }

    let (start, end) = (v * t, v * s);

    if start <= d && d <= end {
        println!("No");
    } else {
        println!("Yes");
    }
}