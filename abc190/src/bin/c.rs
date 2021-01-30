use proconio::{fastout, input};

// todo dish struct add, mut `ball`
// todo loop by choice

#[fastout]
fn main() {
    input! {
        n: i32,
        m: i32,
        cond: [(i32, i32); n],
        k: i32,
        choice: [(i32, i32); k],
    }

    /// ボールを置くパターンを網羅して羅列する
    /// それにはまず「前の質問までの配列を2倍にして置く皿IDを末尾にそれぞれ足す」
    /// をchoiceがなくなるまでやる
    /// Boxにしてここにパターンを表すvecのboxをため込んでいく
    let mut patterns: Vec<&mut Vec<i32>> = vec![];
    for i in 0..k-1 {
        if i == 0 {
            let case = choice.get(i as usize).unwrap();
            let mut case_0 = vec![case.0];
            let mut case_1 = vec![case.1];
            patterns.push(&mut case_0);
            patterns.push(&mut case_1);
        } else {
            // let tmp_patterns = patterns.clone();

            // pattern1
            for accum in patterns.iter_mut() {
                let current = choice.get(i as usize).unwrap();
                accum.push(current.clone().0);
            }

            // pattern2
        }
    }

    println!("{:?}", patterns);
    // println!("{:?}", cond);
    // println!("{:?}", choice);
}
