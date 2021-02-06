use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i64,
        x: i64,
        a: [i64; n],
    }

    let result = a.iter()
        .filter(|&value| value.clone() != x.clone())
        .map(|&value| value.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", result);
}