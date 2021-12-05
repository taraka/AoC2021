#![feature(int_abs_diff)]
use std::io::{self, Read};

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

fn get_map(lines: &Vec<Line>) -> Vec<Vec<usize>> {
    let (x, y) = lines.iter().fold((0, 0), |(x, y), l| {
        (x.max(l.x1).max(l.x2), y.max(l.y1).max(l.y2))
    });

    // This could be so much faster generating the points on the lines and then adding them to the map

    lines.iter().fold(vec![vec![0; y + 1]; x + 1], |map, l| {
        map.iter()
            .enumerate()
            .map(|(x, column)| {
                column
                    .iter()
                    .enumerate()
                    .map(|(y, point)| {
                        if point_is_on_line(x, y, &l) {
                            point + 1
                        } else {
                            *point
                        }
                    })
                    .collect()
            })
            .collect()
    })
}

fn point_is_on_line(x: usize, y: usize, line: &Line) -> bool {
    if line.x1 == line.x2 || line.y1 == line.y2 {
        return x >= line.x1.min(line.x2)
            && x <= line.x1.max(line.x2)
            && y >= line.y1.min(line.y2)
            && y <= line.y1.max(line.y2);
    }

    if line.x1.abs_diff(line.x2) == line.y1.abs_diff(line.y2) {
        let x_step: isize = if line.x1 < line.x2 { 1 } else { -1 };
        let y_step: isize = if line.y1 < line.y2 { 1 } else { -1 };

        for i in 0..=line.x1.abs_diff(line.x2) {
            if x == ((x_step * i as isize) + line.x1 as isize) as usize
                && y == ((y_step * i as isize) + line.y1 as isize) as usize
            {
                return true;
            }
        }
    }

    return false;
}

fn part1(input: &str) -> usize {
    let lines = input
        .lines()
        .map(Line::new)
        .filter(|l| l.x1 == l.x2 || l.y1 == l.y2)
        .collect();

    get_map(&lines)
        .into_iter()
        .flatten()
        .filter(|c| *c > 1)
        .count()
}

fn part2(input: &str) -> usize {
    let lines = input.lines().map(Line::new).collect();

    get_map(&lines)
        .into_iter()
        .flatten()
        .filter(|c| *c > 1)
        .count()
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
