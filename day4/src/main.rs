use std::io::{self, Read};

const CARD_SIZE: usize = 5;

type Value = u32;
type Card = [(Value, bool); CARD_SIZE * CARD_SIZE];

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    println!("Part1: {}", part1(&buffer));

    Ok(())
}

fn part1(input: &str) -> Value {
    let (balls, cards) = parse_input(input);

    for round in 1..=balls.len() {
        if let Some(c) = play_rounds(&balls[..round], &cards) {
            return c.iter().filter(|(_, m)| !*m).map(|(v, _)| *v).sum::<Value>() * balls[round-1];
        }
    }
    panic!("No solution found");
}

fn play_rounds(balls: &[Value], cards: &Vec<Card>) -> Option<Card> {
    cards
        .iter()
        .map(|c| {
            c.map(|(v, m)| {
                if balls.contains(&v) {
                    (v, true)
                } else {
                    (v, m)
                }
            })
        })
        .find(is_winner)
}

fn is_winner(card: &Card) -> bool {
    card.chunks(CARD_SIZE)
        .any(|line| line.iter().all(|(_, m)| *m))
        || (0..CARD_SIZE).any(|column| card.iter().skip(column).step_by(CARD_SIZE).all(|(_, m)| *m))
}

fn parse_input(input: &str) -> (Vec<Value>, Vec<Card>) {
    let (balls_str, cards_str) = input.split_once("\n\n").unwrap();
    (parse_balls(balls_str), parse_cards(cards_str))
}

fn parse_balls(input: &str) -> Vec<Value> {
    input
        .split(",")
        .map(|x| Value::from_str_radix(x, 10).unwrap())
        .collect()
}

fn parse_cards(input: &str) -> Vec<Card> {
    input
        .split("\n\n")
        .map(|cs| {
            cs.lines()
                .flat_map(|l| l.split(" "))
                .filter_map(|x| Value::from_str_radix(x, 10).ok())
                .map(|x| (x, false))
                .collect::<Vec<(Value, bool)>>()
                .try_into()
                .expect("This card doesn't have the right number of values")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7"#;
        assert_eq!(part1(&input), 4512);
    }
}
