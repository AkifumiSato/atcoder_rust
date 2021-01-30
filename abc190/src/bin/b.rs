use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: i32,
        meta: (i32, i32),
        a: [(i32, i32); n],
    }
    let result = a
        .iter()
        .filter(|&(x, y)| x.clone() < meta.0 && y.clone() > meta.1)
        .collect::<Vec<&(i32, i32)>>();
    if result.len() == 0 {
        println!("No");
    } else {
        println!("Yes");
    }
}
