use std::collections::HashMap;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    println!("Part1: \n{}", run(&buffer, 10));
    println!("Part2: \n{}", run(&buffer, 40));

    Ok(())
}

fn run(input: &str, runs: u64) -> u64 {
    let (init, rules) = parse_input(input);
    let counts = (0..runs)
        .fold(init, |acc, _| apply_rules(acc, &rules))
        .into_iter()
        .filter(|((c, _), v)| *v > 0 && *c != ' ')
        .fold(HashMap::new(), |mut map, ((c, _), v)| {
            *map.entry(c).or_insert(0) += v;
            map
        });
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn parse_input(input: &str) -> (HashMap<(char, char), u64>, HashMap<(char, char), char>) {
    let (initial, rules_input) = input.split_once("\n\n").unwrap();
    let rules = rules_input.lines().map(parse_rules).collect();
    (
        format!(" {} ", initial)
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .fold(HashMap::new(), |mut map, c| {
                *map.entry((c[0], c[1])).or_insert(0) += 1;
                map
            }),
        rules,
    )
}

fn parse_rules(line: &str) -> ((char, char), char) {
    let (pair, insert) = line.split_once(" -> ").unwrap();
    let mut chars = pair.chars();
    (
        (chars.next().unwrap(), chars.next().unwrap()),
        insert.chars().next().unwrap(),
    )
}

fn apply_rules(
    mut current: HashMap<(char, char), u64>,
    rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), u64> {
    for ((a, b), v) in current.clone() {
        if v == 0 {
            continue;
        }
        if let Some(c) = rules.get(&(a, b)) {
            *current.entry((a, b)).or_insert(0) -= v;
            *current.entry((a, *c)).or_insert(0) += v;
            *current.entry((*c, b)).or_insert(0) += v;
        }
    }

    current
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
