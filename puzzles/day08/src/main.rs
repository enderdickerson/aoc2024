use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

/// Compute the greatest common divisor using Euclid's algorithm.
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

/// Solve Part 1: Original logic where antinodes appear only if one antenna is double the distance of the other.
fn solve_part1(lines: &[String]) -> usize {
    let (rows, cols) = (lines.len(), if lines.is_empty() { 0 } else { lines[0].len() });
    let mut freq_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    // Collect antenna positions by frequency
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch != '.' {
                freq_map.entry(ch)
                    .or_insert_with(Vec::new)
                    .push((r as i32, c as i32));
            }
        }
    }

    let mut antinodes = HashSet::new();

    // For each frequency
    for positions in freq_map.values() {
        if positions.len() < 2 { continue; }
        let lenp = positions.len();

        // All pairs (i, j), i < j
        for i in 0..lenp {
            for j in (i+1)..lenp {
                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];
                // outside1 = 2 * p1 - p2
                let out1 = (2*r1 - r2, 2*c1 - c2);
                // outside2 = 2 * p2 - p1
                let out2 = (2*r2 - r1, 2*c2 - c1);

                // Insert if in bounds
                if out1.0 >= 0 && out1.0 < rows as i32 && out1.1 >= 0 && out1.1 < cols as i32 {
                    antinodes.insert(out1);
                }
                if out2.0 >= 0 && out2.0 < rows as i32 && out2.1 >= 0 && out2.1 < cols as i32 {
                    antinodes.insert(out2);
                }
            }
        }
    }

    antinodes.len()
}

/// Solve Part 2: Any grid position in line with at least two same-frequency antennas is an antinode.
fn solve_part2(lines: &[String]) -> usize {
    let (rows, cols) = (lines.len(), if lines.is_empty() { 0 } else { lines[0].len() });
    let mut freq_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    // Collect antenna positions by frequency
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch != '.' {
                freq_map.entry(ch)
                    .or_insert_with(Vec::new)
                    .push((r as i32, c as i32));
            }
        }
    }

    let mut antinodes = HashSet::new();

    // For each frequency group, find all pairs and mark collinear cells
    for positions in freq_map.values() {
        if positions.len() < 2 {
            // Single antenna can't form a line, so no new antinodes
            continue;
        }

        // All pairs
        let lenp = positions.len();
        for i in 0..lenp {
            for j in (i+1)..lenp {
                let (r1, c1) = positions[i];
                let (r2, c2) = positions[j];
                if r1 == r2 && c1 == c2 {
                    // Same cell (unlikely if puzzle guaranteed distinct antenna positions).
                    // Skip or continue if that can happen.
                    continue;
                }
                // Step is the smallest vector along that line
                let dr = r2 - r1;
                let dc = c2 - c1;
                let g = gcd(dr, dc);
                let step_r = dr / g;
                let step_c = dc / g;

                // We'll march outward from p1 in direction of (step_r, step_c)
                // until we pass the grid bounds, marking each cell as an antinode.
                // Also march in negative direction from p1,
                // and do the same from p2 in both directions.
                // But to avoid double coverage, a simpler approach:
                //
                //   Mark all cells collinear on line p1..p2 (including beyond them)
                //   If we only need to ensure each cell that sees at least 2 antennas,
                //   we just need the line that covers p1 <-> p2. But the puzzle says
                //   “any grid position exactly in line with at least two antennas,”
                //   so we consider the entire infinite line within the grid in both directions.
                //
                // Start from p1, go forward:
                let mut rr = r1;
                let mut cc = c1;
                // Move backward from p1 until out of bounds
                while rr >= 0 && rr < rows as i32 && cc >= 0 && cc < cols as i32 {
                    antinodes.insert((rr, cc));
                    rr -= step_r;
                    cc -= step_c;
                }
                // Start from p1, go forward
                rr = r1 + step_r;
                cc = c1 + step_c;
                while rr >= 0 && rr < rows as i32 && cc >= 0 && cc < cols as i32 {
                    antinodes.insert((rr, cc));
                    rr += step_r;
                    cc += step_c;
                }

                // Then do the same from p2 in both directions:
                // Actually, it's redundant if we already covered the entire line from p1
                // in both directions. So p2's line is the same line.
                // => The line is the same, so we do not need to do the second pass from p2
                // in both directions.
                // The entire infinite line is covered from the single pass above.
            }
        }
    }

    antinodes.len()
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().filter_map(Result::ok).collect();

    // Part 1
    let p1 = solve_part1(&lines);
    println!("Part 1: {}", p1);

    // Part 2
    let p2 = solve_part2(&lines);
    println!("Part 2: {}", p2);
}
