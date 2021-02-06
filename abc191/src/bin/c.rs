use proconio::{fastout, input};

const MARK: char = '#';

fn empty_cells(cells: Vec<char>) -> bool {
    !cells.iter()
        .any(|&x| x == MARK)
}

fn equal_cells(first: Vec<char>, second: Vec<char>) -> bool {
    first.iter()
        .enumerate()
        .all(|(i, &x)| x.clone() == second.get(i).unwrap().clone())
}

fn find_polygon_number_of_line(cells: Vec<char>) -> i32 {
    let mut result = 0;
    for (i, cell) in cells.iter().enumerate() {
        if i != 0 {
            let prev = cells.get(i - 1).unwrap().clone();
            if prev == cell.clone() {
                result += 1;
            }
        }
    }
    result
}

fn diff_cells(first: Vec<char>, second: Vec<char>) -> i32 {
    let first_left_position = first.iter().cloned().position(|x| x == MARK).unwrap_or_else(|| 0);
    let second_left_position = second.iter().cloned().position(|x| x == MARK).unwrap_or_else(|| 0);
    let first_right_position = first.iter().cloned().rposition(|x| x == MARK).unwrap_or_else(|| 0);
    let second_right_position = second.iter().cloned().rposition(|x| x == MARK).unwrap_or_else(|| 0);

    let left_diff = if first_left_position == second_left_position {
        0
    } else {
        1
    };
    let right_diff = if first_right_position == second_right_position {
        0
    } else {
        1
    };
    let point = left_diff + right_diff;

    match point {
        0 => 0,
        1 => 2,
        2 => 4,
        // 一行に複数の凹凸がある場合
        _ => unreachable!()
    }
}

fn calc_polygon(first: Vec<char>, second: Vec<char>) -> i32 {
    // pattern1 firstにCellがない -> 2
    // pattern2 どちらにもCellはあるが異なる -> 3||4
    // pattern3 全く同じ -> 0
    if empty_cells(first.clone()) || empty_cells(second.clone()) {
        return 2
    } else if equal_cells(first.clone(), second.clone()) {
        return 0
    }
    diff_cells(first.clone(), second.clone())
}

fn main_calc(s: Vec<String>) -> i32 {
    let mut result = 0;
    let data = s.iter()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    for (i, cells) in data.iter().enumerate() {
        if i != 0 {
            let add = calc_polygon(data.get(i - 1).unwrap().clone(), cells.clone());
            result += add;
        }
    }

    result
}

#[fastout]
fn main() {
    input! {
        h_row: i64,
        _w_column: i64,
        s: [String; h_row],
    }

    println!("{}", main_calc(s));
}

#[test]
fn equal_cells_test_1() {
    let first = vec!['.', '#'];
    let second = vec!['.', '#'];
    assert!(equal_cells(first, second));
}

#[test]
fn equal_cells_test_2() {
    let first = vec!['.', '#', '#', '#', '.'];
    let second = vec! ['.', '.', '#', '#', '.'];
    assert!(!equal_cells(first, second));
}

#[test]
fn diff_cells_test() {
    let first = vec!['.', '#', '#', '#', '.'];
    let second = vec! ['.', '.', '#', '#', '.'];
    assert_eq!(diff_cells(first, second), 2);
}

#[test]
fn pattern_1() {
    let s = vec![
        ".....",
        ".###.",
        "..##.",
        ".###.",
        ".....",
    ].iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let result = main_calc(s);
    assert_eq!(result, 8);
}
