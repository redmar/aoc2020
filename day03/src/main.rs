use std::fmt;

#[derive(Debug, PartialEq, Clone)]
enum MapItem {
    T, // #
    O, // .
}

struct TreeMap {
    pub width: usize,
    pub height: usize,
    pub pattern: Vec<Vec<MapItem>>,
}

impl TreeMap {
    fn from_input(input: &str) -> Self {
        fn string_to_map(input: &str) -> Vec<MapItem> {
            input
                .chars()
                .map(|c| match c {
                    '.' => MapItem::O,
                    '#' => MapItem::T,
                    _ => panic!("unexpected input"),
                })
                .collect()
        }

        let pattern: Vec<Vec<MapItem>> = input.lines().map(|line| string_to_map(&line)).collect();
        let width = pattern[0].len();
        let height = pattern.len();
        TreeMap {
            width,
            height,
            pattern,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<MapItem> {
        self.pattern
            .get(y)
            .and_then(|line| line.get(x % self.width))
            .cloned()
    }

    fn tree_count_for_slope(&self, step_x: usize, step_y: usize) -> usize {
        let mut tree_count = 0;
        let mut x = step_x;
        let mut y = step_y;
        loop {
            match self.get(x, y) {
                Some(MapItem::T) => tree_count = tree_count + 1,
                Some(MapItem::O) => (),
                None => break,
            }
            x = x + step_x;
            y = y + step_y;
        }
        tree_count
    }
}

// not really needed but handy to be able to debug the parse/conversion
impl fmt::Display for TreeMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut map_as_string = String::new();

        for row in &self.pattern {
            for col in row {
                match col {
                    MapItem::T => map_as_string.push_str("#"),
                    MapItem::O => map_as_string.push_str("."),
                }
            }
            map_as_string.push_str("\n")
        }
        write!(f, "{}", map_as_string)
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map: TreeMap = TreeMap::from_input(&input);

    println!(
        "solution part 1, tree count for slope right 3, down 1 = {}",
        map.tree_count_for_slope(3, 1)
    );

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    println!(
        "solution part 2, multiply of tree count of slopes {:?} = {}",
        slopes,
        slopes
            .iter()
            .fold(1, |acc, (x, y)| acc * map.tree_count_for_slope(*x, *y))
    );
}
