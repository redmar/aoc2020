mod part1;
mod part2;

fn run_part1(input: &str, debug: bool) -> usize {
    let mut grid = part1::Grid::from_str(input);
    if debug {
        println!("{}occupied_seats_count: {}", grid, grid.occupied_seats_count());
    }

    let mut prev_count = 0;
    loop {
        grid.step();
        let current_count = grid.occupied_seats_count();
        if debug {
            println!("{}occupied_seats_count: {}", grid, current_count);
        }
        if prev_count == current_count {
            break;
        } else {
            prev_count = current_count;
        }
    }
    prev_count
}

fn run_part2(input: &str, debug: bool) -> usize {
    let mut grid = part2::Grid::from_str(input);
    if debug {
        println!("{}occupied_seats_count: {}", grid, grid.occupied_seats_count());
    }

    let mut prev_count = 0;
    loop {
        grid.step();
        let current_count = grid.occupied_seats_count();
        if debug {
            println!("{}occupied_seats_count: {}", grid, current_count);
        }
        if prev_count == current_count {
            break;
        } else {
            prev_count = current_count;
        }
    }
    prev_count
}

fn main() {
    let input = include_str!("../input.txt");

    println!("part1 = {}", run_part1(&input, false));
    println!("part2 = {}", run_part2(&input, false));
}
