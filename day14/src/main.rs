use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, Read};

type CountsMap = HashMap<(char, char), u64>;
type Rules = HashMap<(char, char), char>;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    println!("Part1: {}", run(&buffer, 10));
    println!("Part2: {}", run(&buffer, 40));

    Ok(())
}

fn run(input: &str, runs: u64) -> u64 {
    let (init, rules) = parse_input(input);
    let counts = (0..runs)
        .fold(init, |acc, _| apply_rules(acc, &rules))
        .into_iter()
        .fold(HashMap::new(), |mut map, ((c, _), v)| {
            *map.entry(c).or_insert(0) += v;
            map
        });
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn parse_input(input: &str) -> (CountsMap, Rules) {
    let (initial_str, rules_str) = input.split_once("\n\n").unwrap();
    let rules = rules_str.lines().map(parse_rules).collect();
    let initial =
        format!("{} ", initial_str)
            .chars()
            .tuple_windows()
            .fold(HashMap::new(), |mut map, t| {
                *map.entry(t).or_insert(0) += 1;
                map
            });
    (initial, rules)
}

fn parse_rules(line: &str) -> ((char, char), char) {
    let chars: Vec<char> = line.chars().collect();
    ((chars[0], chars[1]), chars[6])
}

fn apply_rules(current: CountsMap, rules: &Rules) -> CountsMap {
    current.iter().fold(HashMap::new(), |mut map, ((a, b), v)| {
        if let Some(c) = rules.get(&(*a, *b)) {
            *map.entry((*a, *c)).or_insert(0) += v;
            *map.entry((*c, *b)).or_insert(0) += v;
        } else if *b == ' ' {
            map.insert((*a, *b), 1);
        }
        map
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let input1 = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;
        assert_eq!(run(&input1, 10), 1588);
        assert_eq!(run(&input1, 40), 2188189693529);
    }
}
