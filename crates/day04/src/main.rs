use std::fs;

fn get_char(data: &Vec<&str>, row: usize, col: usize) -> char {
    data[row].chars().nth(col).unwrap()
}

fn get_adjacent(data: &Vec<&str>, row: usize, col: usize) -> Vec<(char, usize, usize)> {
    let mut adjacent: Vec<(char, usize, usize)> = Vec::new();
    let rows = data.len();
    let cols = data[0].chars().count();
    // out of bounds
    if row > rows - 1 || col > cols - 1 {
        return adjacent;
    }
    // Previous row
    if row > 0 {
        if col > 0 {
            adjacent.push((get_char(&data, row - 1, col - 1), row - 1, col - 1));
        }
        adjacent.push((get_char(&data, row - 1, col), row - 1, col));
        if col < data.len() - 1 {
            adjacent.push((get_char(&data, row - 1, col + 1), row - 1, col + 1));
        }
    }
    // current row
    if col > 0 {
        adjacent.push((get_char(data, row, col - 1), row, col - 1));
    }
    if col < data[0].len() - 1 {
        adjacent.push((get_char(data, row, col + 1), row, col + 1));
    }
    // next row
    if row < data.len() - 1 {
        if col > 0 {
            adjacent.push((get_char(data, row + 1, col - 1), row + 1, col - 1));
        }
        adjacent.push((get_char(data, row + 1, col), row + 1, col));
        if col < data[0].len() - 1 {
            adjacent.push((get_char(data, row + 1, col + 1), row + 1, col + 1));
        }
    }
    adjacent
}

fn find_accessible_rolls(data: &Vec<&str>) -> Vec<(usize, usize)> {
    let mut rolls: Vec<(usize, usize)> = Vec::new();
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if get_char(&data, i, j) == '@' {
                let adjacent = get_adjacent(data, i, j)
                    .iter()
                    .filter(|d| d.0 == '@')
                    .count();
                if adjacent < 4 {
                    rolls.push((i, j));
                }
            }
        }
    }
    rolls
}

fn main() {
    let input = fs::read_to_string("./crates/day04/src/input.txt")
        .expect("File not found")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let part1 = find_accessible_rolls(&input.iter().map(AsRef::as_ref).collect::<Vec<_>>());
    println!("Part 1: {}", part1.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 10] = [
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    ];

    #[test]
    fn test_example() {
        assert_eq!(find_accessible_rolls(&INPUT.to_vec()).len(), 13);
    }
}
