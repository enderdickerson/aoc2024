use std::io::{self, BufRead};

fn main() {
    let (left_list, right_list) = read_input();

    let part1_answer = solve_part1(&left_list, &right_list);
    println!("Part 1: {}", part1_answer);

    let part2_answer = solve_part2(&left_list, &right_list);
    println!("Part 2: {}", part2_answer);
}

fn read_input() -> (Vec<i64>, Vec<i64>) {
    let stdin = io::stdin();
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line_result in stdin.lock().lines() {
        let line = match line_result {
            Ok(l) => l.trim().to_string(),
            Err(_) => continue,
        };
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split_whitespace();
        let l_str = parts.next();
        let r_str = parts.next();

        if let (Some(l_val), Some(r_val)) = (l_str, r_str) {
            if let (Ok(l_num), Ok(r_num)) = (l_val.parse::<i64>(), r_val.parse::<i64>()) {
                left_list.push(l_num);
                right_list.push(r_num);
            }
        }
    }

    (left_list, right_list)
}

fn solve_part1(left_list: &[i64], right_list: &[i64]) -> i64 {
    let mut left = left_list.to_vec();
    let mut right = right_list.to_vec();

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn solve_part2(left_list: &[i64], right_list: &[i64]) -> i64 {
    // Create a frequency map for the right list
    use std::collections::HashMap;
    let mut right_freq = HashMap::new();

    for &num in right_list {
        *right_freq.entry(num).or_insert(0) += 1;
    }

    // Calculate total similarity score
    let mut similarity_score = 0;
    for &num in left_list {
        if let Some(&count) = right_freq.get(&num) {
            similarity_score += num * count;
        }
    }

    similarity_score
}