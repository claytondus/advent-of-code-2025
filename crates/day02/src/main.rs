use std::env::current_dir;
use std::fs;

fn validate(input: &str) -> bool {
    if input.len() % 2 == 0 {
        let chars = input.chars().collect::<Vec<char>>();
        let chunks = chars
            .chunks(input.len() / 2)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();
        if chunks.first().unwrap().eq(chunks.last().unwrap()) {
            return false;
        };
    }
    true
}

fn validate_part2(input: &str) -> bool {
    let half = input.len() / 2;
    for i in (1..=half).rev() {
        let chars = input.chars().collect::<Vec<char>>();
        let chunks = chars
            .chunks(i)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();
        if chunks.iter().all(|c| c.eq(chunks.first().unwrap())) {
            return false;
        }
    }
    true
}

fn find_invalid(input: &str, part2: bool) -> Vec<String> {
    let mut invalid = Vec::new();
    let splits: Vec<&str> = input.split('-').collect();
    let start = splits[0].parse::<i64>().unwrap();
    let end = splits[1].parse::<i64>().unwrap() + 1;
    for input in start..end {
        let input_str = input.to_string();
        if (part2) {
            if !validate_part2(&input_str) {
                invalid.push(input_str);
            }
        } else {
            if !validate(&input_str) {
                invalid.push(input_str);
            }
        }
    }
    invalid
}

fn find_invalids(input: &str, part2: bool) -> Vec<Vec<String>> {
    input.split(",").map(|i| find_invalid(i, part2)).collect()
}

fn sum_invalid(input: Vec<Vec<String>>) -> i64 {
    input
        .iter()
        .flat_map(|i| i.iter().map(|j| j.parse::<i64>().unwrap()))
        .sum()
}

fn main() {
    println!("{}", current_dir().unwrap().display());
    let input_strings = fs::read_to_string("./crates/day02/src/input.txt")
        .expect("File not found")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    let input: Vec<&str> = input_strings.iter().map(AsRef::as_ref).collect();
    let invalids = find_invalids(input[0], false);
    invalids.iter().for_each(|invalid| {
        println!("{:?}", invalid);
    });
    println!("Part 1: {:?}", sum_invalid(invalids));

    let invalids2 = find_invalids(input[0], true);
    invalids2.iter().for_each(|invalid| {
        println!("{:?}", invalid);
    });
    println!("Part 2: {:?}", sum_invalid(invalids2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11_22() {
        assert_eq!(find_invalid("11-22", false), vec!["11", "22"]);
    }

    #[test]
    fn test_99_115() {
        assert_eq!(find_invalid("99-115", false), vec!["99"]);
    }

    #[test]
    fn test_1188511880_1188511890() {
        assert_eq!(
            find_invalid("1188511880-1188511890", false),
            vec!["1188511885"]
        );
    }

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_example() {
        assert_eq!(
            find_invalids(INPUT, false),
            vec![
                vec!["11", "22"],
                vec!["99"],
                vec!["1010"],
                vec!["1188511885"],
                vec!["222222"],
                vec![],
                vec!["446446"],
                vec!["38593859"],
                vec![],
                vec![],
                vec![]
            ]
        );
    }

    #[test]
    fn test_validate_p2() {
        assert_eq!(validate_part2("11"), false);
        assert_eq!(validate_part2("22"), false);
        assert_eq!(validate_part2("12"), true);
        assert_eq!(validate_part2("38593859"), false);
        assert_eq!(validate_part2("14234"), true);
        assert_eq!(validate_part2("2121212121"), false);
        assert_eq!(validate_part2("2223222"), true);
        assert_eq!(validate_part2("824824824"), false);
        assert_eq!(validate_part2("824824821"), true);
    }

    #[test]
    fn test_example_p2() {
        assert_eq!(
            find_invalids(INPUT, true),
            vec![
                vec!["11", "22"],
                vec!["99", "111"],
                vec!["999", "1010"],
                vec!["1188511885"],
                vec!["222222"],
                vec![],
                vec!["446446"],
                vec!["38593859"],
                vec!["565656"],
                vec!["824824824"],
                vec!["2121212121"]
            ]
        );
    }
}
