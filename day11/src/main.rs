#![feature(stdin_forwarders)]
use itertools::iproduct;

use std::io::{self, Read};
use std::collections::VecDeque;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    println!("Part1: {}", part1(&buffer));
    println!("Part2: {}", part2(&buffer));

    Ok(())
}

fn part2(input: &str) -> u32 {
    let mut grid = parse_input(input);
    let mut i = 0;
    loop {
        i += 1;
        if step(&mut grid) == 100 {
            return i;
        }
    }
}

fn part1(input: &str) -> u32 {
    let mut grid = parse_input(input);
    (0..100).map(|_|step(&mut grid)).sum()
}

fn make_neighbors2(
    y: usize,
    x: usize,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let x_min = if x > 0 { x - 1 } else { 0 };
    let x_max = if x < width - 1 { x + 2 } else { width };
    let y_min = if y > 0 { y - 1 } else { 0 };
    let y_max = if y < height - 1 { y + 2 } else { height };
    (y_min..y_max)
        .flat_map(move |y| (x_min..x_max).map(move |x| (y,x)))
        .filter(move |&coord| coord != (y, x))
}


fn step(grid: &mut Vec<Vec<u32>>) -> u32 {
    let width = grid[0].len();
    let height = grid.len();
    let mut stack: VecDeque<(usize, usize)> = iproduct!(0..height, 0..width).collect();
    while let Some((y, x)) = stack.pop_front() {
        match grid[y][x] {
            10 /* Already flashed */ => (),
            9 => {
                grid[y][x] += 1;
                stack.extend(make_neighbors2(y, x, width, height));
            },
            n => {
                grid[y][x] = n + 1;
            }
        }
    }
    grid.iter_mut()
        .map(|row| row.iter_mut())
        .flatten()
        .filter_map(|x| if *x == 10 { Some(*x = 0) } else { None })
        .count() as u32
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.lines().map(|l| l.chars().filter_map(|c| c.to_digit(10) ).collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;
        assert_eq!(part1(&input), 1656);
        assert_eq!(part2(&input), 195);
    }
}
