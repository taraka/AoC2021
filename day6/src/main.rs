use std::io::{self, Read};
use std::time::Instant;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input: Vec<usize> = buffer
        .split(",")
        .filter_map(|x| usize::from_str_radix(x, 10).ok())
        .collect();

    let now1 = Instant::now();
    let part1 = run(&input, 80);
    println!("Part1: {} in {}us", part1, now1.elapsed().as_micros());

    let now2 = Instant::now();
    let part2 = run(&input, 256);
    println!("Part2: {} in {}us", part2, now2.elapsed().as_micros());

    Ok(())
}

fn create_tally(input: &Vec<usize>) -> [usize; 9] {
    input.iter().fold([0; 9], |mut t, v| { t[*v] += 1; t })
}

fn run(input: &Vec<usize>, num_days: usize) -> usize {

    (0..num_days)
        .fold(create_tally(input), |mut tally, i| {
            tally[(i + 7) % tally.len()] += tally[i % tally.len()];
            tally
        })
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![3, 4, 3, 1, 2];
        assert_eq!(run(&input, 18), 26);
        assert_eq!(run(&input, 80), 5934);
        assert_eq!(run(&input, 256), 26984457539);
    }
}
