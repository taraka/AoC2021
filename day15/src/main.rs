use pathfinding::prelude::{absdiff, astar};
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    println!("Part1: {}", run(&buffer, 1));
    println!("Part2: {}", run(&buffer, 5));

    Ok(())
}

fn run(input: &str, size: u32) -> u32 {
    let map = parse_input(input, size);
    let end = (map.len() - 1, map[0].len() - 1);
    let route = astar(
        &(0, 0),
        |p| neighbors(*p, &map),
        |(x, y)| (absdiff(end.0, *x) + absdiff(end.1, *y)) as u32,
        |p| *p == end,
    )
    .expect("No Route Found");
    route.1
}

fn neighbors((x, y): (usize, usize), map: &Vec<Vec<u32>>) -> Vec<((usize, usize), u32)> {
    let mut points = vec![(x + 1, y), (x, y + 1)];
    if x > 0 {
        points.push((x - 1, y))
    }
    if y > 0 {
        points.push((x, y - 1))
    }
    points
        .iter()
        .filter_map(|(px, py)| Some(((*px, *py), *map.get(*px)?.get(*py)?)))
        .collect()
}

fn parse_input(input: &str, size: u32) -> Vec<Vec<u32>> {
    (0..size)
        .flat_map(|i| {
            input
                .lines()
                .map(|l| {
                    (0..size)
                        .flat_map(|j| {
                            l.chars()
                                .filter_map(|c| c.to_digit(10))
                                .map(|d| ((d + i + j - 1) % 9) + 1)
                                .collect::<Vec<u32>>()
                        })
                        .collect()
                })
                .collect::<Vec<Vec<u32>>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let input1 = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;
        assert_eq!(run(&input1, 1), 40);
        assert_eq!(run(&input1, 5), 315);
    }
}
