use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum GridItem {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub floor: Vec<GridItem>,
    pub next_floor: Vec<GridItem>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for row in self.floor.chunks(self.width as usize) {
            for floor_tile in row {
                match floor_tile {
                    GridItem::EmptySeat => output.push_str("L"),
                    GridItem::OccupiedSeat => output.push_str("#"),
                    GridItem::Floor => output.push_str("."),
                }
            }
            output.push_str("\n");
        }
        write!(f, "{}\n", output)
    }
}

impl Grid {
    pub fn from_str(input: &str) -> Grid {
        let width = input.find('\n').unwrap() as i32;
        let floor = input.chars().fold(Vec::new(), |mut floor_acc, x| {
            match x {
                'L' => floor_acc.push(GridItem::EmptySeat),
                '#' => floor_acc.push(GridItem::OccupiedSeat),
                '.' => floor_acc.push(GridItem::Floor),
                _ => (),
            }
            floor_acc
        });
        let height = (floor.len() / width as usize) as i32;
        Grid { width, height, next_floor: floor.clone(), floor }
    }

    fn get(&self, x: i32, y: i32) -> &GridItem {
        &self.floor[(y * self.width + x) as usize]
    }

    fn set(&mut self, x: i32, y: i32, value: GridItem) {
        self.next_floor[(y * self.width + x) as usize] = value;
    }

    // for all offset directions below, walk each direction and stop when occupied or empty seat
    // -1,1  |  0,1  | 1,1
    // -1,0  |       | 1,0
    // -1,-1 |  0,-1 | 1,-1
    // we pick one of those 'slopes' (for offset_x/offset_y for loops, and then in that direction
    // walk one iteration in that direction until finding a seat, when we're out of bounds of the
    // grid stop with this direction and continue with another direction. (except 0,0)
    //
    fn occupied_seats(&self, x: i32, y: i32) -> i32 {
        let mut occupied_seat_count = 0;
        let offsets: [i32; 3] = [-1, 0, 1];
        for offset_x in offsets.iter() {
            for offset_y in offsets.iter() {
                // skip the middle grid tile, that's us.
                if *offset_x == 0 && *offset_y == 0 { continue; };
                let mut walked_offset: (i32, i32);
                let mut iteration = 1;

                // for each direction:
                loop {
                    // how many iteration steps to walk in this direction
                    walked_offset = (*offset_x * iteration, *offset_y * iteration);

                    let check_x = x + walked_offset.0;
                    let check_y = y + walked_offset.1;
                    if check_x >= 0 && check_x < self.width && check_y >= 0 && check_y < self.height
                    {
                        let seat = self.get(check_x, check_y);
                        if seat == &GridItem::OccupiedSeat {
                            occupied_seat_count += 1;
                            break;
                        } else if seat == &GridItem::EmptySeat {
                            break;
                        }
                    } else {
                        // we're out of bounds, try walking in another (offset) direction
                        break;
                    }

                    iteration += 1;
                }
            }
        }
        occupied_seat_count
    }

    fn step_empty_seat(&mut self, x: i32, y: i32) {
        if self.occupied_seats(x, y) == 0 {
            self.set(x, y, GridItem::OccupiedSeat)
        }
    }

    fn step_occupied_seat(&mut self, x: i32, y: i32) {
        if self.occupied_seats(x, y) >= 5 {
            self.set(x, y, GridItem::EmptySeat)
        }
    }

    fn step_pos(&mut self, x: i32, y: i32) {
        match self.get(x, y) {
            GridItem::EmptySeat => self.step_empty_seat(x, y),
            GridItem::OccupiedSeat => self.step_occupied_seat(x, y),
            GridItem::Floor => (),
        }
    }

    pub fn step(&mut self) {
        self.next_floor = self.floor.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                self.step_pos(x, y);
            }
        }
        self.floor = self.next_floor.clone();
    }

    pub fn occupied_seats_count(&self) -> usize {
        self.floor.iter().filter(|x| **x == GridItem::OccupiedSeat).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input =
            ".##.##.
 #.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.";

        let grid = Grid::from_str(&input);
        assert_eq!(grid.occupied_seats(3, 3), 0);
    }
}

