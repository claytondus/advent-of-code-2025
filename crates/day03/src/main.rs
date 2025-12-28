use iter_first_max::IterFirstMaxExt;
use std::fs;

fn largest_joltage(input: &str) -> u32 {
    let batteries = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    // Find largest battery in any position except last (so it will always be 2 digit)
    let (index, max) = batteries[0..batteries.len() - 1]
        .iter()
        .enumerate()
        .first_max_by_key(|&(_index, &val)| val)
        .unwrap();
    let subarray = batteries[index + 1..].to_vec();
    let max2 = subarray.iter().max().unwrap();
    10 * max + max2
}

fn main() {
    let input_strings = fs::read_to_string("./crates/day03/src/input.txt")
        .expect("File not found")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    let input = input_strings.iter().map(AsRef::as_ref).collect::<Vec<_>>();
    let joltages = input.into_iter().map(largest_joltage).collect::<Vec<_>>();

    let sum1: u32 = joltages.iter().sum();
    println!("Part 1: {}", sum1); //17430
}

#[cfg(test)]
mod tests {
    use crate::largest_joltage;

    #[test]
    fn test_example() {
        assert_eq!(largest_joltage(&"987654321111111".to_string()), 98);
        assert_eq!(largest_joltage(&"811111111111119".to_string()), 89);
        assert_eq!(largest_joltage(&"234234234234278".to_string()), 78);
        assert_eq!(largest_joltage(&"818181911112111".to_string()), 92);
    }
}
