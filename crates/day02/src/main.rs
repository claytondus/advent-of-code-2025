use std::env::current_dir;
use std::fs;

fn validate(input: &str) -> bool {
    if input.len() % 2 == 0 {
        let (head, tail) = input.split_at(input.len() / 2);
        if head.eq(tail) {
            return false;
        };
    }
    true
}

fn find_invalid(input: &str) -> Vec<String> {
    let mut invalid = Vec::new();
    let splits: Vec<&str> = input.split('-').collect();
    let start = splits[0].parse::<i64>().unwrap();
    let end = splits[1].parse::<i64>().unwrap() + 1;
    for input in start..end {
        let input_str = input.to_string();
        if !validate(input_str.as_str()) {
            invalid.push(input_str);
        }
    }
    invalid
}

fn find_invalids(input: &str) -> Vec<Vec<String>> {
    input.split(",").map(find_invalid).collect()
}

fn sum_invalid(input: Vec<Vec<String>>) -> i64 {
    input
        .into_iter()
        .flat_map(|i| i.into_iter().map(|j| j.parse::<i64>().unwrap()))
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
    let invalids = find_invalids(input[0]);
    invalids.iter().for_each(|invalid| {
        println!("{:?}", invalid);
    });
    println!("Part 1: {:?}", sum_invalid(invalids));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_11_22() {
        assert_eq!(find_invalid("11-22"), vec!["11", "22"]);
    }

    #[test]
    fn test_99_115() {
        assert_eq!(find_invalid("99-115"), vec!["99"]);
    }

    #[test]
    fn test_1188511880_1188511890() {
        assert_eq!(find_invalid("1188511880-1188511890"), vec!["1188511885"]);
    }

    #[test]
    fn test_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(
            find_invalids(input),
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
}
