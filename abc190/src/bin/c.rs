use proconio::{fastout, input};

#[derive(Debug, Clone)]
struct Pattern(Vec<i32>);

impl Pattern {
    fn new(id: i32) -> Pattern {
        Pattern(vec![id])
    }

    fn clone_and_add_ball(&self, ball_id: i32) -> Pattern {
        let mut cloned_pattern = self.clone();
        if cloned_pattern.0.contains(&ball_id) {
            return cloned_pattern
        }
        cloned_pattern.0.push(ball_id);
        Pattern(cloned_pattern.0)
    }

    fn judge(&self, a: i32, b: i32) -> bool {
        self.0.contains(&a) && self.0.contains(&b)
    }
}

#[derive(Debug, Clone)]
pub struct Accumulator {
    patterns: Vec<Pattern>
}

impl Accumulator {
    fn new() -> Accumulator {
        Accumulator {
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

    fn merge_pattern(&mut self, target: &mut Accumulator) {
        self.patterns.append(&mut target.patterns);
    }
}

#[fastout]
fn main() {
    input! {
        _n: i32,
        m: i32,
        cond: [(i32, i32); m],
        k: i32,
        choice: [(i32, i32); k],
    }

    let mut accum = Accumulator::new();
    for i in 0..k {
        if i == 0 {
            let case = choice.get(0).unwrap();
            accum.add_initial_pattern(case.0, case.1);
        } else {
            let mut cloned_result = accum.clone();
            let case = choice.get(i as usize).unwrap();

            // 前半部分にcase.0を追加
            accum.add_tail_patterns(case.0);

            // 後半部分にcase.1を追加
            cloned_result.add_tail_patterns(case.1);
            accum.merge_pattern(&mut cloned_result);
        }
    }

    // 条件を満たしているか検査
    let result = accum.patterns
        .iter()
        .map(|p| cond.iter()
            .filter(|(a, b)| p.judge(a.clone(), b.clone()))
            .collect::<Vec<_>>()
            .len())
        .max()
        .unwrap();
    println!("{:?}", result);
}
