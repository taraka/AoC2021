use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    println!("Part1: {}", part1(&buffer));
    println!("Part2: {}", part2(&buffer));
    Ok(())
}

enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn make_direction(line: &str) -> Option<Direction> {
    let (dir, val_str) = line.split_once(" ").unwrap();
    let val = i32::from_str_radix(val_str, 10).ok()?;
    match dir {
        "down" => Some(Direction::Down(val)),
        "up" => Some(Direction::Up(val)),
        "forward" => Some(Direction::Forward(val)),
        _ => panic!("Not a valid dir"),
    }
}

fn move_ship_part1((x, y): (i32, i32), dir: Direction) -> (i32, i32) {
    match dir {
        Direction::Forward(n) => (x + n, y),
        Direction::Down(n) => (x, y + n),
        Direction::Up(n) => (x, y - n),
    }
}

fn part1(input: &str) -> i32 {
    let (x, y) = input
        .lines()
        .filter_map(make_direction)
        .fold((0, 0), move_ship_part1);
    x * y
}

fn move_ship_part2((x, y, aim): (i32, i32, i32), dir: Direction) -> (i32, i32, i32) {
    match dir {
        Direction::Forward(n) => (x + n, y + (n * aim), aim),
        Direction::Down(n) => (x, y, aim + n),
        Direction::Up(n) => (x, y, aim - n),
    }
}

fn part2(input: &str) -> i32 {
    let (x, y, _aim) = input
        .lines()
        .filter_map(make_direction)
        .fold((0, 0, 0), move_ship_part2);
    x * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;
        assert_eq!(part1(&input), 150);

        assert_eq!(part2(&input), 900);
    }
}

//
