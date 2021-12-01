#![feature(stdin_forwarders)]

fn main() {
    let input = std::io::stdin().lines().filter_map(|l| u32::from_str_radix(&l.unwrap(), 10).ok()).collect();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input, 3));
}

pub fn part1(input: &Vec<u32>) -> u32 {
    let mut prev: Option<u32> = None;
    let mut count = 0;

    for current in input.iter() {
        if let Some(p) = prev {
            if p < *current {
                count += 1;
            }
        }

        prev = Some(*current);
    };

    count
}

pub fn part2(input: &Vec<u32>, window_size: usize) -> u32 {
    let mut prev: Option<&[u32]> = None;
    let mut count = 0;

    for current in input.windows(window_size) {
        if let Some(p) = prev {
            if p.iter().sum::<u32>() < current.iter().sum() {
                count += 1;
            }
        }

        prev = Some(current);
    };

    count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(part1(&input), 7);
        assert_eq!(part2(&input, 3), 5);
    }
}