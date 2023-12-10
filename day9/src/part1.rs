use aoc_utils::{
    read_file,
};

fn get_history_derivative(history: &Vec<i64>) -> Vec<i64> {
    let mut out: Vec<i64> = vec![];
    for i in 1..history.len() {
        let diff = history[i] - history[i-1];
        out.push(diff);
    }

    assert!(out.len() == history.len() - 1);
    out
}

fn extrapolate_history(history: Vec<i64>) -> i64 {
    assert!(history.len() > 0);

    let mut extrapolation_column: Vec<i64> = vec![];
    extrapolation_column.push(history[history.len() - 1]);

    let mut hist_current = history;
    loop {
        // Get derviative of history
        hist_current = get_history_derivative(&hist_current);
        if hist_current.clone().into_iter().all(|e| e == 0) {
            break
        }

        extrapolation_column.push(hist_current[hist_current.len() - 1]);
    }

    extrapolation_column.into_iter().fold(0, |acc, val| acc + val)
}

pub fn solve(filepath: &str) -> String {
    let data = read_file(filepath);

    let value_histories: Vec<Vec<i64>> = data.lines()
        .map(|l| l.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect::<Vec<i64>>()).collect();

    let mut next_values: Vec<i64> = vec![];
    for h in value_histories {
        let next = extrapolate_history(h);
        next_values.push(next);
    }

    next_values.into_iter().fold(0, |acc, v| acc + v).to_string()
}
