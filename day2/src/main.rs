use std::io::{self, Read};

type Ship = (i32, i32, i32);

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    println!("Part1: {}", run(&buffer, move_ship_part1));
    println!("Part2: {}", run(&buffer, move_ship_part2));
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
        _ => None,
    }
}

fn move_ship_part1((x, y, _): Ship, dir: Direction) -> Ship {
    match dir {
        Direction::Forward(n) => (x + n, y, 0),
        Direction::Down(n) => (x, y + n, 0),
        Direction::Up(n) => (x, y - n, 0),
    }
}

fn move_ship_part2((x, y, aim): Ship, dir: Direction) -> Ship {
    match dir {
        Direction::Forward(n) => (x + n, y + (n * aim), aim),
        Direction::Down(n) => (x, y, aim + n),
        Direction::Up(n) => (x, y, aim - n),
    }
}

fn run(input: &str, move_fn: fn(Ship, Direction) -> Ship) -> i32 {
    let (x, y, _aim) = input
        .lines()
        .filter_map(make_direction)
        .fold((0, 0, 0), move_fn);
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
        assert_eq!(run(&input, move_ship_part1), 150);

        assert_eq!(run(&input, move_ship_part2), 900);
    }
}
