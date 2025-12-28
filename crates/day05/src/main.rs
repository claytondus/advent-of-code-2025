use std::fs;

fn parse_lines(input: Vec<&str>) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut in_ranges = true;
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();
    for line in input {
        if line.is_empty() {
            in_ranges = false;
            continue;
        }
        if in_ranges {
            let (start, end) = line.split_once("-").unwrap();
            ranges.push((start.parse().unwrap(), end.parse().unwrap()));
        } else {
            ingredients.push(line.parse().unwrap());
        }
    }
    (ranges, ingredients)
}

fn is_fresh(ranges: &Vec<(u64, u64)>, ingredient: &u64) -> bool {
    for range in ranges {
        if ingredient >= &range.0 && ingredient <= &range.1 {
            return true;
        }
    }
    false
}

fn find_fresh(ranges: &Vec<(u64, u64)>, ingredients: Vec<u64>) -> Vec<u64> {
    ingredients
        .into_iter()
        .filter(|i| is_fresh(ranges, i))
        .collect::<Vec<_>>()
}

fn main() {
    let input = fs::read_to_string("./crates/day05/src/input.txt")
        .expect("File not found")
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();

    let inputs = parse_lines(input.iter().map(String::as_str).collect());
    let part1 = find_fresh(&inputs.0, inputs.1).len();

    println!("Part 1: {}", part1); //865
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 11] = [
        "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
    ];

    #[test]
    fn test_parse_lines() {
        assert_eq!(
            parse_lines(INPUT.to_vec()),
            (
                vec![(3, 5), (10, 14), (16, 20), (12, 18)],
                vec![1, 5, 8, 11, 17, 32]
            )
        );
    }

    #[test]
    fn test_find_fresh() {
        let inputs = parse_lines(INPUT.to_vec());
        assert_eq!(find_fresh(&inputs.0, inputs.1), vec![5, 11, 17]);
    }
}
