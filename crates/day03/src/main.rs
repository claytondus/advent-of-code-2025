use iter_first_max::IterFirstMaxExt;
use std::fs;

fn get_batteries(input: &str) -> Vec<u32> {
    input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<u32>>()
}

fn largest_joltage(input: &str) -> u32 {
    let batteries = get_batteries(input);
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

fn largest_joltage_12(input: &str) -> u64 {
    let batteries = get_batteries(input);
    let mut digits: Vec<u32> = Vec::new();
    let mut last_index = 0;
    for i in (0..=11).rev() {
        // Find largest battery in first position excluding last i positions
        let (index, max) = batteries[last_index..=batteries.len() - i - 1]
            .iter()
            .enumerate()
            .first_max_by_key(|&(_index, &val)| val)
            .unwrap();
        last_index += index + 1;
        digits.push(max.clone())
    }
    let mut result: u64 = 0;
    for digit in digits {
        result = result * 10 + digit as u64;
    }
    result
}

fn main() {
    let input_strings = fs::read_to_string("./crates/day03/src/input.txt")
        .expect("File not found")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    let input = input_strings.iter().map(AsRef::as_ref).collect::<Vec<_>>();
    let joltages = input
        .iter()
        .map(|i| largest_joltage(&i))
        .collect::<Vec<_>>();

    let sum1: u32 = joltages.iter().sum();
    println!("Part 1: {}", sum1); //17430

    let joltages_12 = input
        .iter()
        .map(|i| largest_joltage_12(&i))
        .collect::<Vec<_>>();
    let sum2: u64 = joltages_12.iter().sum();
    println!("Part 2: {}", sum2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(largest_joltage(&"987654321111111".to_string()), 98);
        assert_eq!(largest_joltage(&"811111111111119".to_string()), 89);
        assert_eq!(largest_joltage(&"234234234234278".to_string()), 78);
        assert_eq!(largest_joltage(&"818181911112111".to_string()), 92);
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            largest_joltage_12(&"987654321111111".to_string()),
            987654321111
        );
        assert_eq!(
            largest_joltage_12(&"811111111111119".to_string()),
            811111111119
        );
        assert_eq!(
            largest_joltage_12(&"234234234234278".to_string()),
            434234234278
        );
        assert_eq!(
            largest_joltage_12(&"818181911112111".to_string()),
            888911112111
        );
    }
}
