use std::io::{self, BufRead};

/// Generate all combinations of operators for the given length.
/// - If `include_concat` is false, we only generate `+` and `*`.
/// - If `include_concat` is true, we also generate `|` (representing `||`).
fn generate_operator_combinations(length: usize, include_concat: bool) -> Vec<Vec<char>> {
    let base: u32 = if include_concat { 3 } else { 2 };
    let total_combinations = base.pow(length as u32);
    let mut combinations = Vec::with_capacity(total_combinations as usize);

    for mut i in 0..total_combinations {
        let mut combo = Vec::with_capacity(length);
        for _ in 0..length {
            if include_concat {
                // We have three possible operators: +, *, |
                match i % 3 {
                    0 => combo.push('+'),
                    1 => combo.push('*'),
                    2 => combo.push('|'), // Using `|` to represent `||`
                    _ => unreachable!(),
                }
                i /= 3;
            } else {
                // We only have two possible operators: +, *
                if i % 2 == 0 {
                    combo.push('+');
                } else {
                    combo.push('*');
                }
                i /= 2;
            }
        }
        combinations.push(combo);
    }

    combinations
}

/// Evaluate the equation strictly left-to-right.
///
/// For each operator:
/// - `+` => partial_result = partial_result + next_number
/// - `*` => partial_result = partial_result * next_number
/// - `|` => partial_result = concat_digits(partial_result, next_number)
fn evaluate_equation(numbers: &[i64], operators: &[char]) -> i64 {
    let mut partial_result = numbers[0];

    for (i, &op) in operators.iter().enumerate() {
        let next_num = numbers[i + 1];
        match op {
            '+' => {
                partial_result += next_num;
            }
            '*' => {
                partial_result *= next_num;
            }
            '|' => {
                // Concatenate partial_result and next_num as digit strings
                let concatenated = format!("{}{}", partial_result, next_num)
                    .parse::<i64>()
                    .unwrap();
                partial_result = concatenated;
            }
            _ => panic!("Unsupported operator"),
        }
    }

    partial_result
}

/// Process the input equations:
/// - If `include_concat` is false, we only try `+` and `*`.
/// - If `include_concat` is true, we also allow `|` (||).
/// Return the sum of all targets that can be matched by any operator combination.
fn process_equations(lines: &[String], include_concat: bool) -> i64 {
    let mut total_calibration_result = 0;

    for line in lines {
        // Each line is "<target>: <numbers...>"
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue; // Skip malformed lines
        }
        let target: i64 = parts[0].trim().parse().unwrap();
        let numbers: Vec<i64> = parts[1]
            .trim()
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .collect();
        if numbers.is_empty() {
            continue;
        }

        // Generate all combinations of operators, depending on `include_concat`
        let operator_combinations = generate_operator_combinations(numbers.len() - 1, include_concat);

        let mut can_match = false;
        // Try each combination to see if it hits the target
        for operators in operator_combinations {
            let value = evaluate_equation(&numbers, &operators);
            if value == target {
                can_match = true;
                break;
            }
        }

        if can_match {
            total_calibration_result += target;
        }
    }

    total_calibration_result
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();

    // Part 1: Only include +, *
    let part1_result = process_equations(&lines, false);
    println!("Part 1: {}", part1_result);

    // Part 2: Include +, *, and |
    let part2_result = process_equations(&lines, true);
    println!("Part 2: {}", part2_result);
}
