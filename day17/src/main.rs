mod part1;
mod part2;

use part1::Grid3;
use part2::Grid4;

fn main() {
    let input = include_str!("../input.txt");

    let mut grid3 = Grid3::new(input);
    for _ in 1..=6 { grid3.cycle(); }
    println!("Part1 solution: cubes in active state = {}", grid3.in_active_state());

    let mut grid4 = Grid4::new(input);
    for _ in 1..=6 { grid4.cycle(); }
    println!("Part2 solution: cubes in active state = {}", grid4.in_active_state());
}