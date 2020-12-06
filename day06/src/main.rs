use std::collections::HashSet;
use std::iter::FromIterator;

fn part1_sum_of_total_unique_answered(groups: &Vec<&str>) -> usize {
    // total unique questions answered per group
    groups
        .iter()
        .map(|l| {
            // unique questions answered per group
            l.chars()
                .filter(char::is_ascii_alphabetic)
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

fn part2_sum_of_same_answered(groups: &Vec<&str>) -> usize {
    // pretty verbose way to go from ["ab\nabc", "a\na\na"] to
    // [[{'a','b'},{'a','b','c'}],[{'a'},{'a'},{'a'}]] where {} is a HashSet.
    // took too much time to get it cleaner
    let groups_of_hashsets_groups: Vec<Vec<HashSet<char>>> = groups
        .iter()
        .map(|l| l.lines().collect::<Vec<&str>>())
        .map(|mut k| {
            k.iter_mut()
                .map(|q| HashSet::<char>::from_iter(q.chars()))
                .collect::<Vec<HashSet<char>>>()
        })
        .collect();

    // intersection per group
    groups_of_hashsets_groups
        .iter()
        .map(|g| {
            let mut g_iter = g.iter();
            let mut x: HashSet<char> = g_iter.next().unwrap().clone();
            while let Some(other) = g_iter.next() {
                x = x.intersection(&other).copied().collect();
            }
            x.len()
        })
        .sum()
}

fn main() {
    let groups: Vec<&str> = include_str!("../input.txt").split("\n\n").collect();
    println!("part1 = {:?}", part1_sum_of_total_unique_answered(&groups));
    println!("part2 = {}", part2_sum_of_same_answered(&groups));
}
