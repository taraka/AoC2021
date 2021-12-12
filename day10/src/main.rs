#![feature(stdin_forwarders)]
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
        .filter_map(process_line_part1)
        .sum()
}

fn part2(input: &str) -> usize {
    let mut scores: Vec<usize> = input.lines().filter_map(process_line_part2).collect();
    scores.sort();
    scores[scores.len()/2]
}

fn process_line_part1(line: &str) -> Option<usize> {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '<' | '{' => stack.push(c),
            ')' | ']' | '>' | '}' => {
                if stack.pop()? != get_matching(c) {
                    return get_score(c);
                }
            }
            _ => panic!("Unknown char: {}", c)
        }
    }
    None
}

fn process_line_part2(line: &str) -> Option<usize> {
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '(' | '[' | '<' | '{' => stack.push(c),
            ')' | ']' | '>' | '}' => {
                if stack.pop()? != get_matching(c) {
                    return None;
                }
            }
            _ => panic!("Unknown char: {}", c)
        }
    }

    Some(incomplete_score(&stack))
}

fn incomplete_score(stack: &Vec<char>) -> usize {
    let mut total = 0;

    for c in stack.iter().rev() {
        total *= 5;
        total += match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("Unknown char: {}", c)
        }
    }

    total
}

fn get_matching(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ =>  panic!("Unknown char: {}", c)
    }
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
        assert_eq!(part2(&input), 288957);
    }
}
