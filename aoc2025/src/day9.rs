static EXAMPLE: &str = r#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Point2d(i64, i64);

impl Point2d {
    pub fn new(source: &str) -> Self {
        let parsed: Vec<i64> = source.split(',').map(|x| str::parse(x).unwrap()).collect();
        Self(parsed[0], parsed[1])
    }

    pub fn manhattan_distance(self: &Self, other: &Self) -> u64 {
        i64::abs_diff(self.0, other.0) as u64 + i64::abs_diff(self.0, other.0) as u64
    }
}

fn rect_area(a: &Point2d, b: &Point2d) -> u64 {
    (1 + i64::abs_diff(a.0, b.0) as u64) * (i64::abs_diff(a.1, b.1) + 1) as u64
}

fn parse_points(input: &str) -> Vec<Point2d> {
    input
        .split('\n')
        .filter(|x| x.len() > 0)
        .map(Point2d::new)
        .collect()
}

fn part1(input: &str) -> u64 {
    let tiles = parse_points(input);
    dbg!(&tiles);

    let pairs = combinations::Combinations::new(tiles, 2);
    let areas = pairs.map(|pair| {
        let res = (pair.clone(), rect_area(&pair[0], &pair[1]));
        // dbg!(&res);
        res
    });
    let max_area = areas.max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    // dbg!(&max_area);
    max_area.1
}

fn main() {
    println!("example");
    println!("part1={}", part1(EXAMPLE));
    println!("input");
    let day9 = std::fs::read_to_string("inputs/day9.txt").unwrap();
    println!("part1={}", part1(&day9));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE), 50)
    }
}
