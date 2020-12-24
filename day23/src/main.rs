use std::collections::HashMap;
use std::time::Instant;

struct CrabGame {
    current_cup: u32,
    ring: HashMap<u32, u32>,
    max_cup: u32,
}

impl CrabGame {
    fn new_part1(elems: &Vec<u32>) -> Self {
        let mut next_elems = elems.clone();
        next_elems.rotate_left(1);
        let mut ring = HashMap::new();
        for (cup, next_cup) in elems.iter().zip(next_elems.iter()) {
            ring.insert(*cup, *next_cup);
        }

        Self {
            ring,
            current_cup: *elems.first().unwrap(),
            max_cup: *elems.iter().max().unwrap(),
        }
    }

    fn new_part2(elems: &mut Vec<u32>) -> Self {
        elems.reserve(1_000_000);
        for idx in (elems.iter().max().unwrap() + 1)..=1_000_000 {
            elems.push(idx as u32);
        }

        let mut next_elems = elems.clone();
        next_elems.rotate_left(1);

        let mut ring = HashMap::with_capacity(1_000_000);
        for (cup, next_cup) in elems.iter().zip(next_elems.iter()) {
            ring.insert(*cup, *next_cup);
        }

        let mut current = elems.first().unwrap();
        for _ in elems.iter().take(15) {
            let next = ring.get(current).unwrap();
            current = next;
        }

        Self {
            ring,
            current_cup: *elems.first().unwrap(),
            max_cup: *elems.iter().max().unwrap(),
        }
    }

    fn print_part1_solution_format(&self, start_cup: Option<u32>) -> String {
        let start_cup = start_cup.unwrap_or(self.current_cup);
        let mut out = String::new();
        let mut cup: (u32, u32) = (start_cup, *self.ring.get(&start_cup).unwrap());
        for idx in 0..self.ring.keys().len() {
            if idx != 0 {
                out.push_str(&format!("{}", cup.0));
            }
            cup = (cup.1, *self.ring.get(&cup.1).unwrap())
        }
        out
    }

    fn part2_solution(&self) -> u64 {
        let e1 = *self.ring.get(&1).unwrap();
        let e2 = *self.ring.get(&e1).unwrap();
        e1 as u64 * e2 as u64
    }

    fn make_move(&mut self) {
        let removed_cups = self.remove_next_three_cups(self.current_cup);
        self.move_three_cups_to(self.get_destination_cup(removed_cups), removed_cups);
        self.current_cup = *self.ring.get(&self.current_cup).unwrap();
    }

    fn remove_next_three_cups(&mut self, current_cup: u32) -> (u32, u32, u32) {
        let next_of_current_cup = *self.ring.get(&current_cup).unwrap();
        let e1 = { self.ring.remove_entry(&next_of_current_cup).unwrap() };
        let e2 = { self.ring.remove_entry(&e1.1).unwrap() };
        let e3 = { self.ring.remove_entry(&e2.1).unwrap() };
        self.ring.insert(current_cup, e3.1);
        (e1.0, e2.0, e3.0)
    }

    fn move_three_cups_to(&mut self, destination_cup: u32, three_cups: (u32, u32, u32)) {
        let next_of_dest_cup = *self.ring.get(&destination_cup).unwrap();
        self.ring.insert(destination_cup, three_cups.0);
        self.ring.insert(three_cups.0, three_cups.1);
        self.ring.insert(three_cups.1, three_cups.2);
        self.ring.insert(three_cups.2, next_of_dest_cup);
    }

    fn get_destination_cup(&self, three_cups: (u32, u32, u32)) -> u32 {
        let wrapped_decrement = |destination_cup| {
            let destination_cup = destination_cup - 1;
            if destination_cup == 0 {
                self.max_cup
            } else {
                destination_cup
            }
        };

        let mut destination_cup = wrapped_decrement(self.current_cup);
        while CrabGame::is_in_three_cups(destination_cup, three_cups) {
            destination_cup = wrapped_decrement(destination_cup);
        }
        destination_cup
    }

    fn is_in_three_cups(value: u32, three_cups: (u32, u32, u32)) -> bool {
        value == three_cups.0 || value == three_cups.1 || value == three_cups.2
    }
}

fn main() {
    let elems = vec![8, 7, 1, 3, 6, 9, 4, 5, 2];
    let mut game1 = CrabGame::new_part1(&elems);
    for _ in 1..=100 { game1.make_move() }
    println!("Part1 solution: {}", game1.print_part1_solution_format(Some(1)));

    let mut elems2 = elems.clone();
    let mut game2 = CrabGame::new_part2(&mut elems2);
    let now = Instant::now();
    {
        for _ in 1..=10_000_000 { game2.make_move() }
        println!("Part2 solution: {}", game2.part2_solution());
    }
    println!("Elapsed: {} secs", now.elapsed().as_secs_f32());
}