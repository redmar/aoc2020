use std::collections::HashSet;
use std::iter::FromIterator;

fn lines_of_strings_to_vec(input: &str) -> Vec<i32> {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i32>().unwrap())
        .collect()
}

fn find_pair_that_adds_up_to_2020(numbers: &Vec<i32>) -> Option<(i32, i32)> {
    for x in 0..numbers.len() {
        for y in 0..numbers.len() {
            if numbers[x] + numbers[y] == 2020 {
                return Some((numbers[x], numbers[y]));
            }
        }
    }
    None
}

fn find_triple_that_adds_up_to_2020(numbers: &Vec<i32>) -> Option<(i32, i32, i32)> {
    for x in 0..numbers.len() {
        for y in 0..numbers.len() {
            for z in 0..numbers.len() {
                if numbers[x] + numbers[y] + numbers[z] == 2020 {
                    return Some((numbers[x], numbers[y], numbers[z]));
                }
            }
        }
    }
    None
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let numbers: Vec<i32> = lines_of_strings_to_vec(&input);

    match find_pair_that_adds_up_to_2020(&numbers) {
        Some((x, y)) => println!("part1 (double that adds up to 2020) = {}", x * y),
        None => panic!("ohno! no pair that adds up to 2020 can be found in the input!"),
    }

    match find_triple_that_adds_up_to_2020(&numbers) {
        Some((x, y, z)) => println!("part2 (triple that adds up to 2020) = {}", x * y * z),
        None => panic!("ohno! no triple that adds up to 2020 can be found in the input!"),
    }

    dbg!(find_entries(&numbers));
}

// seen this solution later which is pretty nice:
// https://twitter.com/imjasonmiller/status/1333748461209055233?s=20
// TIL .find_map
//

fn find_entries(xs: &[i32]) -> Option<i32> {
    let entries = HashSet::<i32>::from_iter(xs.iter().copied());
    entries
        .iter()
        .find_map(|x| entries.get(&(2020 - x)).map(|y| (*x, *y)))
        .map(|(x, y)| x * y)
}
