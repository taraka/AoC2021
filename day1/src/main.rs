


fn main() {
    println!("Hello, world!");
}

pub fn part1(input: Vec<u32>) -> u32 {
    let mut prev: Option<u32> = None;
    let mut count = 0;

    for current in input.into_iter() {
        if let Some(p) = prev {
            if p < current {
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
        assert_eq!(part1(input), 7);
    }
}