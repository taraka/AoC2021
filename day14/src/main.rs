use std::io::{self, Read};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    println!("Part1: \n{}", part1(&buffer));
    // println!("Part2: \n{}", part2(&buffer));

    Ok(())
}

fn part1(input: &str) -> usize {
    let (init, rules) = parse_input(input);
    let output = (0..10).fold(init, |acc, _| apply_rules(acc, &rules) );
    let counts = output.iter().fold(HashMap::new(), |mut map, c| { *map.entry(c).or_insert(0) += 1; map } );
    counts.values().max().unwrap() - counts.values().min().unwrap()
}


fn parse_input(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let (initial, rules_input) = input.split_once("\n\n").unwrap();
    let rules = rules_input.lines().map(parse_rules).collect();
    (initial.chars().collect(), rules)
}

fn parse_rules(line: &str) -> ((char, char), char) {
    let (pair, insert) = line.split_once(" -> ").unwrap();
    let mut chars = pair.chars();
    ((chars.next().unwrap(), chars.next().unwrap()), insert.chars().next().unwrap())
}

fn apply_rules(current: Vec<char>, rules: &HashMap<(char, char), char>) -> Vec<char> {
    let mut new: Vec<char> = current.windows(2).flat_map(|p| match rules.get(&(p[0], p[1])) {
        Some(c) => vec![p[0], *c],
        None => vec![p[0]],
    }).collect();

    new.push(*current.iter().last().unwrap());

    new
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
        assert_eq!(part1(&input1), 1588);
    }
}
