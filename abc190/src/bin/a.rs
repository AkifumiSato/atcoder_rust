use proconio::{fastout, input};

enum Winner {
    Takahashi,
    Aoki,
}

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }
    let mut winner = Winner::Takahashi;

    if c == 0 {
        if a == b {
            winner = Winner::Aoki;
        }
    } else {
        if (a + 1) == b {
            winner = Winner::Aoki;
        }
    }

    if a < b {
        winner = Winner::Aoki;
    }

    match winner {
        Winner::Takahashi => println!("Takahashi"),
        Winner::Aoki => println!("Aoki"),
    }
}
