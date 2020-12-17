use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
struct Pos4(i32, i32, i32, i32);

#[derive(Debug, Clone)]
pub(crate) struct Grid4 {
    cubes: HashMap<Pos4, bool>,
    next_cubes: HashMap<Pos4, bool>,
    x_range: (i32, i32),
    y_range: (i32, i32),
    z_range: (i32, i32),
    w_range: (i32, i32),
}

impl Grid4 {
    pub fn new(input: &str) -> Self {
        let size = input.lines().next().unwrap().chars().count() as i32;
        let x_range = (0, size);
        let y_range = (0, size);
        let z_range = (0, 1);
        let w_range = (0, 1);

        let mut cubes = HashMap::new();
        for (y, line) in (0..size).zip(input.lines()) {
            for (x, c) in (0..size).zip(line.chars()) {
                cubes.insert(Pos4(x, y, 0, 0), c == '#');
            }
        }
        let next_cubes = cubes.clone();

        Self { cubes, next_cubes, x_range, y_range, z_range, w_range }
    }

    pub(crate) fn in_active_state(&self) -> u32 {
        self.cubes.values().fold(0, |mut acc, &active| {
            if active { acc += 1 };
            acc
        })
    }

    fn neighbors_turned_on(&self, pos: Pos4) -> u32 {
        let mut total = 0;
        let range = [-1, 0, 1];
        for w in range.iter() {
            for z in range.iter() {
                for y in range.iter() {
                    for x in range.iter() {
                        if *x == 0 && *y == 0 && *z == 0 && *w == 0 { continue; }
                        let val = *self.cubes.get(&Pos4(pos.0 + *x, pos.1 + *y, pos.2 + *z, pos.3 + *w)).unwrap_or(&false);
                        if val { total += 1 }
                    }
                }
            }
        }
        total
    }

    pub(crate) fn cycle(&mut self) {
        self.next_cubes = HashMap::new();

        // fugly but works
        self.w_range = (self.w_range.0 - 1, self.w_range.1 + 1);
        self.z_range = (self.z_range.0 - 1, self.z_range.1 + 1);
        self.x_range = (self.x_range.0 - 1, self.x_range.1 + 1);
        self.y_range = (self.y_range.0 - 1, self.y_range.1 + 1);

        for w in self.w_range.0..self.w_range.1 {
            for z in self.z_range.0..self.z_range.1 {
                for y in self.y_range.0..self.y_range.1 {
                    for x in self.x_range.0..self.x_range.1 {
                        let cube_pos = Pos4(x, y, z, w);
                        let cube_is_active = *self.cubes.get(&cube_pos).unwrap_or(&false);
                        let active_neighbors = self.neighbors_turned_on(Pos4(x, y, z, w));
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
        }
        self.cubes = self.next_cubes.clone();
    }
}

impl fmt::Display for Grid4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        for w in self.w_range.0..self.w_range.1 {
            for z in self.z_range.0..self.z_range.1 {
                let mut z_slice = format!("z={} w={}\n", z, w);

                for y in self.y_range.0..self.y_range.1 {
                    for x in self.x_range.0..self.x_range.1 {
                        z_slice += if *self.cubes.get(&Pos4(x, y, z, w)).unwrap_or(&false) { "#" } else { "." }
                    }
                    z_slice += "\n";
                }
                s += &*z_slice;
                s += "\n";
            }
        }

        write!(f, "{}", s)
    }
}
