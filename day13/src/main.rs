use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    println!("Part1: \n{}", part1(&buffer));
    println!("Part2: \n{}", part2(&buffer));

    Ok(())
}

fn part1(input: &str) -> usize {
    let (points, folds) = parse_input(input);
    let folded = folds
        .iter()
        .take(1)
        .fold(points, |acc, (axis, v)| fold(&acc, *axis, *v));
    folded.len()
}

fn part2(input: &str) -> String {
    let (points, folds) = parse_input(input);
    let folded = folds
        .iter()
        .fold(points, |acc, (axis, v)| fold(&acc, *axis, *v));
    point_map(&folded)
}

fn parse_input(input: &str) -> (HashSet<(usize, usize)>, Vec<(char, usize)>) {
    let (points_input, folds_input) = input.split_once("\n\n").unwrap();
    let points = points_input.lines().map(parse_point).collect();
    let folds = folds_input.lines().map(parse_fold).collect();
    (points, folds)
}

fn parse_point(line: &str) -> (usize, usize) {
    let (x, y) = line.split_once(",").unwrap();
    (
        usize::from_str_radix(x, 10).unwrap(),
        usize::from_str_radix(y, 10).unwrap(),
    )
}

fn parse_fold(line: &str) -> (char, usize) {
    let (axis, v) = line.split(" ").last().unwrap().split_once("=").unwrap();
    (
        axis.chars().next().unwrap(),
        usize::from_str_radix(v, 10).unwrap(),
    )
}

fn point_map(points: &HashSet<(usize, usize)>) -> String {
    let x_max = points.iter().map(|(x, _)| x).max().unwrap();
    let y_max = points.iter().map(|(_, y)| y).max().unwrap();

    let mut output = String::new();

    for y in 0..=*y_max {
        for x in 0..=*x_max {
            if points.contains(&(x, y)) {
                output.push_str("#");
            } else {
                output.push_str(".");
            }
        }
        output.push_str("\n");
    }

    output
}

fn fold(points: &HashSet<(usize, usize)>, axis: char, value: usize) -> HashSet<(usize, usize)> {
    if axis == 'x' {
        return points
            .iter()
            .map(|(x, y)| (if *x > value { value - (x - value) } else { *x }, *y))
            .collect();
    } else if axis == 'y' {
        return points
            .iter()
            .map(|(x, y)| (*x, if *y > value { value - (y - value) } else { *y }))
            .collect();
    } else {
        panic!("Unknown axis: {}", axis);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let input1 = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;
        assert_eq!(part1(&input1), 17);
        assert_eq!(
            part2(&input1),
            r#"#####
#...#
#...#
#...#
#####
"#
        );
    }
}
