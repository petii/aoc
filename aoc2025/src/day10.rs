// day10.rs

use std::fmt::Display;

static EXAMPLE: &str = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

struct MachineState(Vec<bool>);

impl MachineState {
    pub fn all_off(light_count: usize) -> Self {
        Self(vec![false; light_count])
    }

    pub fn parse(state_str: &str) -> Self {
        let unparenthesed = state_str.replace("[", "").replace("]", "");
        MachineState(
            unparenthesed
                .chars()
                .map(|c| if c == '.' { false } else { true })
                .collect(),
        )
    }

    pub fn new(states: Vec<bool>) -> Self {
        Self(states)
    }
}

impl Display for MachineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state_str = format!(
            "[{}]",
            (&self.0)
                .into_iter()
                .map(|state| if *state { '#' } else { '.' })
                .collect::<String>()
        );
        f.write_str(&state_str)
    }
}

impl core::fmt::Debug for MachineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self)[..])
    }
}

struct Button(Vec<usize>);

impl Button {
    pub fn parse(tup_str: &str) -> Self {
        let unparenthesed = tup_str.replace("(", "").replace(")", "");
        Self(
            unparenthesed
                .split(',')
                .map(|num_str| str::parse::<usize>(num_str).unwrap())
                .collect(),
        )
    }
}

impl Display for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &(&self.0)
                .into_iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )
    }
}

impl core::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self))
    }
}

#[derive(Debug)]
struct Machine {
    lights: MachineState,
    on_state: MachineState,
    buttons: Vec<Button>,
    joltages: Vec<u32>,
}

impl Machine {
    pub fn new(line: &str) -> Self {
        let part_count = line.split(' ').count();
        let mut split_line = line.split(' ');
        let lights_str = split_line.next().unwrap();
        let on_state = MachineState::parse(lights_str);
        let buttons_strs = split_line.take(part_count - 2);
        let joltages_str = line.split(' ').last().unwrap();

        Self {
            lights: MachineState::all_off(on_state.0.len()),
            on_state,
            buttons: buttons_strs.map(Button::parse).collect(),
            joltages: joltages_str
                .replace("{", "")
                .replace("}", "")
                .split(",")
                .map(|c| str::parse::<u32>(c).unwrap())
                .collect(),
        }
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .split('\n')
        .filter(|x| x.len() > 0)
        .map(Machine::new)
        .collect()
}

fn part1(input: &str) -> u32 {
    let machines = parse_input(input);
    dbg!(&machines[0]);
    machines.len().try_into().unwrap()
}

fn main() {
    println!("example");
    println!("part1={}", part1(EXAMPLE));
    println!("input");
    let day10 = std::fs::read_to_string("inputs/day10.txt").unwrap();
    println!("part1={}", part1(&day10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 7)
    }
}
