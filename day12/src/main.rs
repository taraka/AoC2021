use std::io::{self, Read};
use std::collections::{HashMap, HashSet};

type Paths<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    println!("Part1: {}", part1(&buffer));
    println!("Part2: {}", part2(&buffer));

    Ok(())
}

fn part1(input: &str) -> usize {
    let map = parse_input(input);
    find_paths("start", &map, &HashSet::from(["start"]), 0, false)
}

fn part2(input: &str) -> usize {
    let map = parse_input(input);
    find_paths("start", &map, &HashSet::from(["start"]), 0, true)
}

fn parse_input(input: &str) -> Paths {
    input
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .fold(HashMap::new(), |mut map, (a,b)| {
            map.entry(a).or_insert(HashSet::new()).insert(b);
            map.entry(b).or_insert(HashSet::new()).insert(a);
            map
        })
}

fn find_paths(current: &str, map: &Paths, visited: &HashSet<&str>, depth: usize, allow_double: bool) -> usize {
    if current == "end" {
        return 1;
    }
    map[current].iter().fold(0, |acc, p| {
        let mut new_visited = visited.clone();
        new_visited.insert(p);
        if (*p).chars().all(|c| c.is_ascii_uppercase()) {
            return acc + find_paths(p, map, visited, depth, allow_double);
        } else if !visited.contains(p) {
            return acc + find_paths(p, map, &new_visited, depth, allow_double);
        } else if allow_double && *p != "start" && *p != "end" {
            return acc + find_paths(p, map, visited, depth, false);
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small() {
        let input1 = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;
        assert_eq!(part1(&input1), 10);
        assert_eq!(part2(&input1), 36);

    }    #[test]
    fn test_medium() {
        let input2 = r#"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"#;
        assert_eq!(part1(&input2), 19);
        assert_eq!(part2(&input2), 103);
    }

    #[test]
    fn test_part1() {
        let input3 = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;
        assert_eq!(part1(&input3), 226);
        assert_eq!(part2(&input3), 3509);
    }
}
