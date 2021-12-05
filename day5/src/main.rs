#![feature(int_abs_diff)]
use std::io::{self, Read};
use std::collections::HashSet;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    println!("Part1: {}", part1(&buffer));
    println!("Part2: {}", part2(&buffer));

    Ok(())
}

#[derive(Debug)]
struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

impl Line {
    fn new(line: &str) -> Self {
        let (p1, p2) = line.split_once(" -> ").unwrap();
        let (x1, y1) = p1.split_once(",").unwrap();
        let (x2, y2) = p2.split_once(",").unwrap();

        Self {
            x1: usize::from_str_radix(x1, 10).unwrap(),
            y1: usize::from_str_radix(y1, 10).unwrap(),
            x2: usize::from_str_radix(x2, 10).unwrap(),
            y2: usize::from_str_radix(y2, 10).unwrap(),
        }
    }
}

fn points_on_line(line: &Line) -> Vec<(usize, usize)> {
    if line.x1 == line.x2 {
        return (line.y1.min(line.y2)..=line.y1.max(line.y2)).map(|y| (line.x1, y)).collect();
    }

    if line.y1 == line.y2 {
        return (line.x1.min(line.x2)..=line.x1.max(line.x2)).map(|x| (x, line.y1)).collect();
    }

    if line.x1.abs_diff(line.x2) == line.y1.abs_diff(line.y2) {
        let x_step: isize = if line.x1 < line.x2 { 1 } else { -1 };
        let y_step: isize = if line.y1 < line.y2 { 1 } else { -1 };

        return (0..=line.x1.abs_diff(line.x2)).map(|i| (((x_step * i as isize) + line.x1 as isize) as usize, ((y_step * i as isize) + line.y1 as isize) as usize)).collect();
    }

    return vec![];
}

fn part1(input: &str) -> usize {
    let points: Vec<(usize, usize)> = input.lines().map(Line::new).filter(|l| l.x1 == l.x2 || l.y1 == l.y2).flat_map(|l| points_on_line(&l)).collect();
    let uniq_points: HashSet<(usize, usize)> = points.iter().copied().collect();
    points.len() - uniq_points.len()
}

fn part2(input: &str) -> usize {
    let points: Vec<(usize, usize)> = input.lines().map(Line::new).flat_map(|l| points_on_line(&l)).collect();
    let mut points_count: HashMap<(usize, usize), usize> = points.iter().map(|p| (*p, 0)).collect();

    for p in points {
        *points_count.get_mut(&p).unwrap() += 1;
    }

    points_count.values().filter(|c| **c > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;
        assert_eq!(part1(&input), 5);
        assert_eq!(part2(&input), 12);
    }
}
