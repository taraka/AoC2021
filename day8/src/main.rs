#![feature(stdin_forwarders)]
use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    println!("Part1: {}", part1(&buffer));
    println!("Part2: {}", part2(&buffer));

    Ok(())
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|l| l.split_once(" | ").unwrap().1)
        .flat_map(|l| l.split(" "))
        .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
        .count()
}

fn part2(input: &str) -> usize {
    input.lines().map(process_line).sum()
}

fn process_line(line: &str) -> usize {
    let (usp, display) = line.split_once(" | ").unwrap();

    let counts = usp
        .chars()
        .filter(|c| *c != ' ')
        .fold(HashMap::new(), |mut counts, c| {
            *counts.entry(c).or_insert(0) += 1;
            counts
        });

    display
        .split(" ")
        .map(|w| w.chars().map(|c| counts.get(&c).unwrap()).sum())
        .map(|v| match v {
            49 => 8,
            39 => 3,
            45 => 9,
            30 => 4,
            25 => 7,
            41 => 6,
            17 => 1,
            37 => 5,
            34 => 2,
            42 => 0,

            _ => panic!("I don't know that one: {}", v),
        })
        .fold(0, |acc, v| (acc * 10) + v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;
        assert_eq!(part1(&input), 26);
        assert_eq!(part2(&input), 61229);
    }
}
