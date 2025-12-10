// day8.rs
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

static EXAMPLE: &str = r#"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

#[derive(Debug, PartialOrd, Eq, Clone, PartialEq, Hash, Ord)]
struct Point3d(i128, i128, i128);
impl Point3d {
    pub fn new(coords: Vec<i128>) -> Self {
        Self(coords[0], coords[1], coords[2])
    }

    pub fn distance(&self, other: &Self) -> u128 {
        ((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2) + (self.2 - other.2).pow(2))
            .isqrt()
            .try_into()
            .unwrap()
    }
}

fn parse_input(input: &str) -> Vec<Point3d> {
    input
        .split('\n')
        .filter(|row| row.chars().count() > 0)
        .map(|row| {
            Point3d::new(
                row.split(',')
                    .map(|coord_str| str::parse::<i128>(coord_str).unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}

fn part1(input: &str, connections: usize) -> u128 {
    let points = parse_input(input);
    let mut distances_for_pair = HashMap::new();
    let mut points_for_distance: BTreeMap<u128, HashSet<_>> = BTreeMap::new();
    let mut distances = BTreeSet::new();
    for pair_v in combinations::Combinations::new(points.clone(), 2) {
        let [ref a, ref b] = pair_v[..] else { todo!() };
        let pair = (a.clone(), b.clone());
        let distance = a.distance(&b);
        distances.insert(distance);

        distances_for_pair.insert((a.clone(), b.clone()), distance);
        distances_for_pair.insert((b.clone(), a.clone()), distance);

        match points_for_distance.get_mut(&distance) {
            Some(pairs) => {
                pairs.insert(pair);
            }
            None => {
                let mut new_set = HashSet::new();
                new_set.insert(pair);
                points_for_distance.insert(distance, new_set);
            }
        }
    }

    let mut circuits = BTreeMap::<usize, HashSet<Point3d>>::new();
    let mut point_to_circuit = HashMap::new();
    for (i, p) in points.into_iter().enumerate() {
        point_to_circuit.insert(p.clone(), i);
        let mut ps = HashSet::new();
        ps.insert(p);
        circuits.insert(i, ps);
    }

    // distances is ordered since is a BTreeSet
    dbg![distances.len()];
    let smallest_dists = distances.iter();
    let pairs_for_smallest_dists =
        smallest_dists.map(|dist| points_for_distance.get(dist).unwrap());

    let mut made_connections = 0;
    for pair_set in pairs_for_smallest_dists {
        for (a, b) in pair_set {
            let a_circuit = point_to_circuit.get(&a).unwrap().clone();
            let b_circuit = point_to_circuit.get(&b).unwrap().clone();
            if a_circuit != b_circuit {
                // merge!
                let b_ps = circuits.get(&b_circuit).unwrap().clone();
                let a_ps = circuits.get_mut(&a_circuit).unwrap();
                for point in b_ps {
                    a_ps.insert(point.clone());
                    point_to_circuit.insert(point.clone(), a_circuit);
                }
                circuits.remove(&b_circuit);
            }
            made_connections += 1;
        }
        if made_connections >= connections {
            break;
        }
    }
    let mut circuit_sizes = Vec::new();
    for (_c, ps) in &circuits {
        circuit_sizes.push(ps.len() as u128);
    }
    circuit_sizes.sort_by(|a, b| b.cmp(a));
    dbg!(
        circuit_sizes.len(),
        &circuit_sizes
            .clone()
            .into_iter()
            .take(5)
            .collect::<Vec<_>>()
    );
    circuit_sizes
        .into_iter()
        .take(3)
        .fold(1, |acc, points| acc * points)
}

fn part2(input: &str) -> i128 {
    let points = parse_input(input);
    let mut distances_for_pair = HashMap::new();
    let mut points_for_distance: BTreeMap<u128, HashSet<_>> = BTreeMap::new();
    let mut distances = BTreeSet::new();
    for pair_v in combinations::Combinations::new(points.clone(), 2) {
        let [ref a, ref b] = pair_v[..] else { todo!() };
        let pair = (a.clone(), b.clone());
        let distance = a.distance(&b);
        distances.insert(distance);

        distances_for_pair.insert((a.clone(), b.clone()), distance);
        distances_for_pair.insert((b.clone(), a.clone()), distance);

        match points_for_distance.get_mut(&distance) {
            Some(pairs) => {
                pairs.insert(pair);
            }
            None => {
                let mut new_set = HashSet::new();
                new_set.insert(pair);
                points_for_distance.insert(distance, new_set);
            }
        }
    }

    let mut circuits = BTreeMap::<usize, HashSet<Point3d>>::new();
    let mut point_to_circuit = HashMap::new();
    for (i, p) in points.into_iter().enumerate() {
        point_to_circuit.insert(p.clone(), i);
        let mut ps = HashSet::new();
        ps.insert(p);
        circuits.insert(i, ps);
    }

    // distances is ordered since is a BTreeSet
    dbg![distances.len()];
    let smallest_dists = distances.iter();
    let pairs_for_smallest_dists =
        smallest_dists.map(|dist| points_for_distance.get(dist).unwrap());

    let mut latest_pair = (Point3d(0, 0, 0), Point3d(0, 0, 0));
    for pair_set in pairs_for_smallest_dists {
        for (a, b) in pair_set {
            let a_circuit = point_to_circuit.get(&a).unwrap().clone();
            let b_circuit = point_to_circuit.get(&b).unwrap().clone();
            if a_circuit != b_circuit {
                // merge!
                let b_ps = circuits.get(&b_circuit).unwrap().clone();
                let a_ps = circuits.get_mut(&a_circuit).unwrap();
                for point in b_ps {
                    a_ps.insert(point.clone());
                    point_to_circuit.insert(point.clone(), a_circuit);
                }
                circuits.remove(&b_circuit);
                if circuits.len() == 1 {
                    latest_pair = (a.clone(), b.clone());
                    break;
                }
            }
        }
        if circuits.len() == 1 {
            break;
        }
    }
    dbg![&latest_pair];

    latest_pair.0.0 * latest_pair.1.0
}

fn main() {
    println!("example:\n{}", EXAMPLE);
    println!("part 1 = {}", part1(EXAMPLE, 10));
    println!("part 2 = {}", part2(EXAMPLE));
    println!("input");
    let day8 = std::fs::read_to_string("inputs/day8.txt").unwrap();
    println!("part 1 = {}", part1(&day8, 1000));
    println!("part 2 = {}", part2(&day8));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq![part1(EXAMPLE, 10), 40];
    }

    #[test]
    fn example_part2() {
        assert_eq![part2(EXAMPLE), 25272];
    }
}
