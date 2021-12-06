use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let input: Vec<usize> = buffer
        .split(",")
        .filter_map(|x| usize::from_str_radix(x, 10).ok())
        .collect();

    println!("Part1: {}", run(&input, 80));
    println!("Part2: {}", run(&input, 256));

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
