#![feature(stdin_forwarders)]

fn main() {
    let input = std::io::stdin()
        .lines()
        .filter_map(|l| u32::from_str_radix(&l.unwrap(), 10).ok())
        .collect();
    println!("Part1: {}", both_parts(&input, 1));
    println!("Part2: {}", both_parts(&input, 3));
}

pub fn both_parts(input: &Vec<u32>, window_size: usize) -> u32 {
    input
        .windows(window_size + 1)
        .filter(|v| v.iter().rev().skip(1).sum::<u32>() < v.iter().skip(1).sum())
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(both_parts(&input, 1), 7);
        assert_eq!(both_parts(&input, 3), 5);
    }
}
