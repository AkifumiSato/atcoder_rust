use proconio::{fastout, input};

#[derive(Debug, Clone)]
struct Pattern {
    dished_id_list: Vec<i32>
}

impl Pattern {
    fn new(id: i32) -> Pattern {
        Pattern {
            dished_id_list: vec![id],
        }
    }

    fn clone_and_add_ball(&self, ball_id: i32) -> Pattern {
        let mut dished_id_list = self.dished_id_list.clone();
        if !dished_id_list.contains(&ball_id) {
            dished_id_list.push(ball_id);
        }
        Pattern {
            dished_id_list,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CalcResult {
    patterns: Vec<Pattern>
}

impl CalcResult {
    fn new() -> CalcResult {
        CalcResult {
            patterns: vec![],
        }
    }

    fn add_initial_pattern(&mut self, id1: i32, id2: i32) {
        self.patterns.push(Pattern::new(id1));
        self.patterns.push(Pattern::new(id2));
    }

    fn add_tail_patterns(&mut self, id: i32) {
        let patterns = self.patterns
            .iter()
            .map(|p| p.clone_and_add_ball(id))
            .collect::<Vec<Pattern>>();
        self.patterns = patterns;
    }

    fn merge_pattern(&mut self, target: &mut CalcResult) {
        self.patterns.append(&mut target.patterns);
    }
}

#[fastout]
fn main() {
    input! {
        n: i32,
        _m: i32,
        _cond: [(i32, i32); n],
        k: i32,
        choice: [(i32, i32); k],
    }

    // ボールを置くパターンを網羅して羅列する
    // それにはまず「前の質問までの配列を2倍にして置く皿IDを末尾にそれぞれ足す」
    // をchoiceがなくなるまでやる
    let mut result = CalcResult::new();
    for i in 0..k {
        if i == 0 {
            let case = choice.get(0).unwrap();
            result.add_initial_pattern(case.0, case.1);
        } else {
            let mut cloned_result = result.clone();
            //     .iter()
            //     .clone()
            //     .map(|x| x.clone())
            //     .collect::<Vec<Pattern>>();
            let case = choice.get(i as usize).unwrap();

            // 前半部分にcase.0を追加
            result.add_tail_patterns(case.0);

            // 後半部分にcase.1を追加
            cloned_result.add_tail_patterns(case.1);
            result.merge_pattern(&mut cloned_result);
        }
    }

    // todo 条件を満たしているか検査
    println!("{:?}", output);
}
