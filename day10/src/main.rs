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
        .filter_map(process_line)
        .sum()
}

fn part2(input: &str) -> usize {
    input.lines().filter_map(process_line).sum()
}

fn process_line(line: &str) -> Option<usize> {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '<' | '{' => stack.push(c),
            ')' => {
                if stack.pop()? != '(' {
                    return get_score(c);
                }
            }
            ']' => {
                if stack.pop()? != '[' {
                    return get_score(c);
                }
            }
            '>' => {
                if stack.pop()? != '<' {
                    return get_score(c);
                }
            }
            '}' => {
                if stack.pop()? != '{' {
                    return get_score(c);
                }
            }
            _ => panic!("Unknown char: {}", c)
        }
    }
    None
}

fn get_score(c: char) -> Option<usize> {
    match c {
        ')' => Some(3),
        ']' => Some(57),
        '}' => Some(1197),
        '>' => Some(25137),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;
        assert_eq!(part1(&input), 26397);
        // assert_eq!(part2(&input), 61229);
    }
}
