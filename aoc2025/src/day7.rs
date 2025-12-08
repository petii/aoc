static EXAMPLE: &str = r#"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
"#;

fn parse_input(input: &str) -> impl Iterator<Item = &str> {
    input.split('\n').filter(|row| row.chars().count() > 0)
}

fn step_simulation(rows: Vec<String>) -> Option<(Vec<String>, u32, Vec<Vec<u64>>)> {
    let mut next_state = vec![rows[0].to_string()];
    let mut ways_to_reach = vec![vec![0; rows[0].len()]];
    let mut changes = 0;
    let mut splits = 0;
    let mut previous_row_reaches = ways_to_reach[0].clone();
    for win in rows[..].windows(2) {
        let mut second_row: Vec<_> = win[1].chars().collect();
        let mut row_reaches = previous_row_reaches.clone();
        for (i, cell) in win[0].chars().enumerate() {
            let cell_below = win[1].chars().nth(i).unwrap();
            if cell == 'S' {
                if cell_below == '.' {
                    second_row[i] = '|';
                    changes += 1;
                }
                row_reaches[i] += 1;
            }
            if cell == '|' {
                if cell_below == '.' {
                    second_row[i] = '|';
                    changes += 1;
                } else if cell_below == '^' {
                    if i > 0 && second_row[i - 1] == '.' {
                        second_row[i - 1] = '|';
                        changes += 1;
                    }
                    if i < second_row.len() - 1 && second_row[i + 1] == '.' {
                        second_row[i + 1] = '|';
                        changes += 1;
                    }
                    splits += 1;
                    if i > 0 {
                        row_reaches[i - 1] += previous_row_reaches[i];
                    }
                    if i < row_reaches.len() - 1 {
                        row_reaches[i + 1] += previous_row_reaches[i];
                    }
                    row_reaches[i] = 0;
                }
            }
        }
        next_state.push(second_row.iter().collect());
        ways_to_reach.push(row_reaches.clone());
        previous_row_reaches = row_reaches;
    }

    if changes > 0 {
        return Some((next_state, splits, ways_to_reach));
    }
    None
}

fn part1(input: &str, debug: bool) -> u32 {
    let mut rows: Vec<_> = parse_input(input).map(|s| s.to_string()).collect();
    if debug {
        dbg!(&rows);
    }
    let mut last_splits = 0;
    while let Some((next_state, splits, _ways_to_reach)) = step_simulation(rows) {
        if debug {
            dbg!(&next_state);
        }
        rows = next_state;
        last_splits = splits;
    }
    last_splits
}

fn part2(input: &str, debug: bool) -> u64 {
    let mut rows: Vec<_> = parse_input(input).map(|s| s.to_string()).collect();

    let mut last_counts = vec![];
    let mut total_timelines = 0;
    while let Some((next_state, _splits, ways_to_reach)) = step_simulation(rows) {
        rows = next_state;
        total_timelines = ways_to_reach
            .last()
            .unwrap()
            .into_iter()
            .fold(0, |acc, v| acc + v);
        last_counts = ways_to_reach
    }
    if debug {
        let ways_to_reach = last_counts
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|v| format!("{v:2}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect::<Vec<_>>();
        dbg!(ways_to_reach);
        dbg![total_timelines];
    }
    total_timelines
}

fn main() {
    println!("example:\n{}", EXAMPLE);
    println!("part 1 = {}", part1(EXAMPLE, true));
    println!("part 2 = {}", part2(EXAMPLE, true));
    println!("input");
    let day7 = std::fs::read_to_string("inputs/day7.txt").unwrap();
    println!("part 1 = {}", part1(&day7, false));
    println!("part 2 = {}", part2(&day7, false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = EXAMPLE;
        let result = part1(input, true);
        assert_eq!(result, 21);
    }

    #[test]
    fn example_part2() {
        let input = EXAMPLE;
        let result = part2(input, true);
        assert_eq!(result, 40);
    }
}
