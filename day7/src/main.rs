use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input: Vec<i32> = buffer
        .split(",")
        .filter_map(|x| i32::from_str_radix(x, 10).ok())
        .collect();

    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));

    Ok(())
}

fn part1(input: &Vec<i32>) -> i32 {
    (*input.iter().min().unwrap()..*input.iter().max().unwrap())
        .map(|pos| input.iter().map(|x| (pos - x).abs()).sum())
        .min()
        .unwrap()
}
fn part2(input: &Vec<i32>) -> i32 {
    (*input.iter().min().unwrap()..*input.iter().max().unwrap())
        .map(|pos| {
            input
                .iter()
                .map(|x| (pos - x).abs())
                .map(|n| (n * (n + 1)) / 2)
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        assert_eq!(part1(&input), 37);
        assert_eq!(part2(&input), 168);
    }
}
