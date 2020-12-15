use std::collections::HashMap;

#[derive(Debug)]
struct ElvesMemoryGame {
    last_item: u32,
    round: u32,
    history: HashMap<u32, (u32, u32)>,
}

impl ElvesMemoryGame {
    fn new(starting_numbers: &Vec<u32>) -> Self {
        let round = starting_numbers.len();
        let last_item = starting_numbers[round - 1];
        let mut history = HashMap::new();
        for (idx, number) in starting_numbers.iter().enumerate() {
            history.insert(*number, (0, idx as u32 + 1));
        }
        Self { round: round as u32, last_item, history }
    }

    fn round(&mut self, idx: usize) -> Option<u32> {
        self.take(idx - self.round as usize).last()
    }
}

impl Iterator for ElvesMemoryGame {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.round += 1;
        match self.history.get(&self.last_item).copied() {
            Some((prev, curr)) => {
                if curr != 0 && prev != 0 {
                    self.last_item = curr - prev;
                } else {
                    self.last_item = 0;
                }
            }
            None => {
                println!("{} = {} not found", self.round, self.last_item);
                self.last_item = 0;
            }
        }
        let entry = self.history.entry(self.last_item).or_insert((0, 0));
        entry.0 = entry.1;
        entry.1 = self.round;
        Some(self.last_item)
    }
}

// tried to do this one in the form of an rust iterator
fn main() {
    let input = vec![11, 0, 1, 10, 5, 19];

    let mut mem1 = ElvesMemoryGame::new(&input);
    println!("Part1 2020th pos for {:?} = {:?}",
             &input, mem1.round(2020).unwrap());

    let mut mem2 = ElvesMemoryGame::new(&input);
    println!("Part2 30_000_000th pos for {:?} = {:?}",
             &input, mem2.round(30_000_000).unwrap());
}
