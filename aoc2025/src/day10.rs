// day10.rs

use std::collections::HashSet;
use std::{fmt::Display, vec};

static EXAMPLE: &str = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#;

#[derive(Clone)]
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

    pub fn distance(&self, other: &Self) -> usize {
        let mut diffs = 0;
        for (a, b) in (&self.0).into_iter().zip(&other.0) {
            if *a != *b {
                diffs += 1;
            }
        }
        diffs
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

#[derive(Clone)]
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
        f.write_str(&format!(
            "btn({})",
            &(&self.0)
                .into_iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(","),
        ))
    }
}

impl core::fmt::Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self))
    }
}

#[derive(Debug, Clone)]
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

    pub fn button_press(&self, b: &Button) -> MachineState {
        let mut state = self.lights.0.clone();
        for light_index in &b.0 {
            state[*light_index] = !state[*light_index];
        }
        MachineState::new(state)
    }

    pub fn after_button_press(&self, b: &Button) -> Self {
        let mut new_machine = self.clone();
        new_machine.lights = self.button_press(b);
        new_machine
    }

    pub fn is_on(&self) -> bool {
        self.lights.distance(&self.on_state) == 0
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .split('\n')
        .filter(|x| x.len() > 0)
        .map(Machine::new)
        .collect()
}

fn simulate_button_presses_except(m: &Machine, b: Option<&Button>) -> Vec<(Machine, Button, bool)> {
    (&m.buttons)
        .into_iter()
        .filter(|current_b| {
            if let Some(last_b) = b {
                return current_b.0 != last_b.0;
            };
            true
        })
        .map(|b| {
            let new_state = m.after_button_press(&b);
            let is_on = new_state.is_on();
            (new_state, b.clone(), is_on)
        })
        .collect::<Vec<_>>()
}

fn button_presses_multiverse(machine: &Machine) -> usize {
    let mut level = 0;
    let mut states_explored = HashSet::new();
    let mut states_to_explore = simulate_button_presses_except(machine, None);
    let mut any_on = false;
    if any_on {
        return level;
    }
    while !any_on && states_to_explore.len() > 0 {
        level += 1;
        any_on = (&states_to_explore).into_iter().any(|s| s.2);

        let mut next_level_states = vec![];
        for (machine, last_button_press, is_on) in states_to_explore {
            if states_explored.contains(&machine.lights.0) {
                continue;
            }
            if is_on {
                any_on = true;
            }
            let mut new_states_this_machine =
                simulate_button_presses_except(&machine, Some(&last_button_press));
            next_level_states.append(&mut new_states_this_machine);
            states_explored.insert(machine.lights.0);
        }
        dbg![&level];
        states_to_explore = next_level_states;
    }
    level
}

fn part1(input: &str) -> u32 {
    let machines = parse_input(input);
    // dbg!(&machines[0]);
    let mut acc = 0 as u32;
    for m in machines {
        let min_presses = button_presses_multiverse(&m) as u32;
        acc += min_presses;
        dbg![m, min_presses];
    }
    acc
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
