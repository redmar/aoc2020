use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
struct Pos3(i32, i32, i32);

#[derive(Debug, Clone)]
pub(crate) struct Grid3 {
    cubes: HashMap<Pos3, bool>,
    next_cubes: HashMap<Pos3, bool>,
    x_range: (i32, i32),
    y_range: (i32, i32),
    z_range: (i32, i32),
}

impl Grid3 {
    pub fn new(input: &str) -> Self {
        let size = input.lines().next().unwrap().chars().count() as i32;
        let x_range = (0, size);
        let y_range = (0, size);
        let z_range = (0, 1);

        let mut cubes = HashMap::new();
        for (y, line) in (0..size).zip(input.lines()) {
            for (x, c) in (0..size).zip(line.chars()) {
                cubes.insert(Pos3(x, y, 0), c == '#');
            }
        }
        let next_cubes = cubes.clone();

        Self { cubes, next_cubes, x_range, y_range, z_range }
    }

    pub(crate) fn in_active_state(&self) -> u32 {
        self.cubes.values().fold(0, |mut acc, &active| {
            if active { acc += 1 };
            acc
        })
    }

    fn neighbors_turned_on(&self, pos: Pos3) -> u32 {
        let mut total = 0;
        let range = [-1, 0, 1];
        for z in range.iter() {
            for y in range.iter() {
                for x in range.iter() {
                    if *x == 0 && *y == 0 && *z == 0 { continue; }
                    let val = *self.cubes.get(&Pos3(pos.0 + *x, pos.1 + *y, pos.2 + *z)).unwrap_or(&false);
                    if val { total += 1 }
                }
            }
        }
        total
    }

    pub(crate) fn cycle(&mut self) {
        self.next_cubes = HashMap::new();

        // fugly but works
        self.z_range = (self.z_range.0 - 1, self.z_range.1 + 1);
        self.x_range = (self.x_range.0 - 1, self.x_range.1 + 1);
        self.y_range = (self.y_range.0 - 1, self.y_range.1 + 1);

        for z in self.z_range.0..self.z_range.1 {
            for y in self.y_range.0..self.y_range.1 {
                for x in self.x_range.0..self.x_range.1 {
                    let cube_pos = Pos3(x, y, z);
                    let cube_is_active = *self.cubes.get(&cube_pos).unwrap_or(&false);
                    let active_neighbors = self.neighbors_turned_on(Pos3(x, y, z));
                    match cube_is_active {
                        true => {
                            if active_neighbors == 2 || active_neighbors == 3 {
                                self.next_cubes.insert(cube_pos, true);
                            } else {
                                self.next_cubes.insert(cube_pos, false);
                            }
                        }
                        false => {
                            if active_neighbors == 3 {
                                self.next_cubes.insert(cube_pos, true);
                            } else {
                                self.next_cubes.insert(cube_pos, false);
                            }
                        }
                    }
                }
            }
        }
        self.cubes = self.next_cubes.clone();
    }
}

impl fmt::Display for Grid3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        for z in self.z_range.0..self.z_range.1 {
            let mut z_slice = format!("z={}\n", z);

            for y in self.y_range.0..self.y_range.1 {
                for x in self.x_range.0..self.x_range.1 {
                    z_slice += if *self.cubes.get(&Pos3(x, y, z)).unwrap_or(&false) { "#" } else { "." }
                }
                z_slice += "\n";
            }
            s += &*z_slice;
            s += "\n";
        }

        write!(f, "{}", s)
    }
}
