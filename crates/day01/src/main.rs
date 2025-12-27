/*
--- Day 1: Secret Entrance ---
The Elves have good news and bad news.

The good news is that they've discovered project management! This has given them the tools they need to prevent their usual Christmas emergency. For example, they now know that the North Pole decorations need to be finished soon so that other critical tasks can start on time.

The bad news is that they've realized they have a different emergency: according to their resource planning, none of them have any time left to decorate the North Pole!

To save Christmas, the Elves need you to finish decorating the North Pole by December 12th.

Collect stars by solving puzzles. Two puzzles will be made available on each day; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You arrive at the secret entrance to the North Pole base ready to start decorating. Unfortunately, the password seems to have been changed, so you can't get in. A document taped to the wall helpfully explains:

"Due to new security protocols, the password is locked in the safe below. Please see the attached document for the new combination."

The safe has a dial with only an arrow on it; around the dial are the numbers 0 through 99 in order. As you turn the dial, it makes a small click noise as it reaches each number.

The attached document (your puzzle input) contains a sequence of rotations, one per line, which tell you how to open the safe. A rotation starts with an L or R which indicates whether the rotation should be to the left (toward lower numbers) or to the right (toward higher numbers). Then, the rotation has a distance value which indicates how many clicks the dial should be rotated in that direction.

So, if the dial were pointing at 11, a rotation of R8 would cause the dial to point at 19. After that, a rotation of L19 would cause it to point at 0.

Because the dial is a circle, turning the dial left from 0 one click makes it point at 99. Similarly, turning the dial right from 99 one click makes it point at 0.

So, if the dial were pointing at 5, a rotation of L10 would cause it to point at 95. After that, a rotation of R5 could cause it to point at 0.

The dial starts by pointing at 50.

You could follow the instructions, but your recent required official North Pole secret entrance security training seminar taught you that the safe is actually a decoy. The actual password is the number of times the dial is left pointing at 0 after any rotation in the sequence.

For example, suppose the attached document contained the following rotations:

L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
Following these rotations would cause the dial to move as follows:

The dial starts by pointing at 50.
The dial is rotated L68 to point at 82.
The dial is rotated L30 to point at 52.
The dial is rotated R48 to point at 0.
The dial is rotated L5 to point at 95.
The dial is rotated R60 to point at 55.
The dial is rotated L55 to point at 0.
The dial is rotated L1 to point at 99.
The dial is rotated L99 to point at 0.
The dial is rotated R14 to point at 14.
The dial is rotated L82 to point at 32.
Because the dial points at 0 a total of three times during this process, the password in this example is 3.

Analyze the rotations in your attached document. What's the actual password to open the door?
 */
use std::env::current_dir;
use std::fs;

fn rotate(start: i32, direction: i32, distance: i32, part2: bool) -> (i32, i32) {
    let mut zeros = 0;
    let mut end = (start + (distance * direction)) % 100;
    if part2 {
        // Add full rotation zeros
        zeros += distance / 100;
    }
    // Position ending at zero
    if end == 0 {
        println!("Position is 0, Adding zero");
        zeros += 1;
    }

    // Position crossed zero
    if end < 0 {
        end += 100;
        if part2 && start != 0 && distance < 100 {
            zeros += 1;
        }
    } else if start + (direction * distance) > 100 && distance < 100 {
        if part2 {
            zeros += 1;
        }
    }
    (end, zeros)
}

fn password(rotations: &Vec<&str>, part2: bool) -> (i32, i32) {
    let mut zeros = 0;
    let mut position = 50;

    for s in rotations {
        let (dir, dist) = s.split_at(1);
        let distance = dist.parse::<i32>().unwrap();
        let direction: i32 = match dir {
            "L" => -1,
            "R" => 1,
            _ => 0,
        };
        let (end, zeros_added) = rotate(position, direction, distance, part2);
        println!("{} {} to {}, added {} zeros", position, s, end, zeros_added);

        position = end;
        zeros += zeros_added;
    }

    (zeros, position)
}

fn main() {
    println!("{}", current_dir().unwrap().display());
    let input_strings = fs::read_to_string("./crates/day01/src/input.txt")
        .expect("File not found")
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();
    let input = input_strings.iter().map(AsRef::as_ref).collect();

    let password1 = password(&input, false);
    println!("Part 1: {:?}", password1);

    let password2 = password(&input, true);
    println!("Part 2: {:?}", password2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static [&str] = &[
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];

    #[test]
    fn test_password_p1() {
        let password = password(&Vec::from(INPUT), false);

        assert_eq!(password, (3, 32));
    }

    #[test]
    fn test_password_p2() {
        let password = password(&Vec::from(INPUT), true);

        assert_eq!(password, (6, 32));
    }

    #[test]
    fn test_rotate_l68() {
        assert_eq!(rotate(50, -1, 68, false), (82, 0));
        assert_eq!(rotate(50, -1, 68, true), (82, 1))
    }

    #[test]
    fn test_rotate_l30() {
        assert_eq!(rotate(82, -1, 30, false), (52, 0));
        assert_eq!(rotate(82, -1, 30, true), (52, 0))
    }

    #[test]
    fn test_rotate_r48() {
        assert_eq!(rotate(52, 1, 48, false), (0, 1));
        assert_eq!(rotate(52, 1, 48, true), (0, 1))
    }

    #[test]
    fn test_rotate_l5() {
        assert_eq!(rotate(0, -1, 5, false), (95, 0));
        assert_eq!(rotate(0, -1, 5, true), (95, 0))
    }

    #[test]
    fn test_rotate_r60() {
        assert_eq!(rotate(95, 1, 60, false), (55, 0));
        assert_eq!(rotate(95, 1, 60, true), (55, 1));
    }

    #[test]
    fn test_rotate_l55() {
        assert_eq!(rotate(55, -1, 55, false), (0, 1));
        assert_eq!(rotate(55, -1, 55, true), (0, 1))
    }

    #[test]
    fn test_rotate_l1() {
        assert_eq!(rotate(0, -1, 1, false), (99, 0));
        assert_eq!(rotate(0, -1, 1, true), (99, 0));
    }

    #[test]
    fn test_rotate_l99() {
        assert_eq!(rotate(99, -1, 99, false), (0, 1));
        assert_eq!(rotate(99, -1, 99, true), (0, 1));
    }

    #[test]
    fn test_rotate_r14() {
        assert_eq!(rotate(0, 1, 14, false), (14, 0));
        assert_eq!(rotate(0, 1, 14, true), (14, 0))
    }

    #[test]
    fn test_rotate_l82() {
        assert_eq!(rotate(14, -1, 82, false), (32, 0));
        assert_eq!(rotate(14, -1, 82, true), (32, 1))
    }

    #[test]
    fn test_rotate_l1000() {
        assert_eq!(rotate(50, -1, 1000, false), (50, 0));
        assert_eq!(rotate(50, -1, 1000, true), (50, 10))
    }
}
