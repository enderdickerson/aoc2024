use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines()
        .filter_map(|l| l.ok())
        .collect();

    // Part 1: Count how many are safe without the dampener
    let safe_count_part1 = lines.iter()
        .map(|line| {
            let levels: Vec<i64> = line.split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();
            is_report_safe(&levels)
        })
        .filter(|&safe| safe)
        .count();

    // Part 2: Count how many are safe with the dampener
    let safe_count_part2 = lines.iter()
        .map(|line| {
            let levels: Vec<i64> = line.split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();
            is_report_safe_with_dampener(&levels)
        })
        .filter(|&safe| safe)
        .count();

    println!("Part 1: {}", safe_count_part1);
    println!("Part 2: {}", safe_count_part2);
}

fn is_report_safe_with_dampener(levels: &[i64]) -> bool {
    if is_report_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut modified = levels.to_vec();
        modified.remove(i);
        if is_report_safe(&modified) {
            return true;
        }
    }

    false
}

fn is_report_safe(levels: &[i64]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let diffs: Vec<i64> = levels.windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect();

    let first_diff = diffs[0];
    if first_diff == 0 {
        return false;
    }

    let all_positive = first_diff > 0;
    for &d in &diffs {
        if (all_positive && d <= 0) || (!all_positive && d >= 0) {
            return false;
        }
        let diff_abs = d.abs();
        if diff_abs < 1 || diff_abs > 3 {
            return false;
        }
    }

    true
}
