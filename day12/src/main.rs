type Position = (i32, i32, i32);

fn part1_travel_one(input: (char, i32), prev_pos: Position) -> Position {
    let (x, y, d) = prev_pos;

    match input {
        ('N', v) => (x, y + v, d),
        ('S', v) => (x, y - v, d),
        ('E', v) => (x + v, y, d),
        ('W', v) => (x - v, y, d),

        ('L', v) => (x, y, d + v),
        ('R', v) => (x, y, d - v),

        ('F', v) => {
            match (d as i32).rem_euclid(360) {
                0 => (x + v, y, d),
                90 => (x, y + v, d),
                180 => (x - v, y, d),
                270 => (x, y - v, d),
                _ => panic!("should not happen")
            }
        }

        _ => panic!("should not happen")
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Waypoint { x: i32, y: i32 }

#[derive(Debug, Copy, Clone, PartialEq)]
struct Ship { x: i32, y: i32 }

fn part2_travel_one(input: (char, i32), w: Waypoint, s: Ship) -> (Waypoint, Ship) {
    match input {
        ('N', v) => (Waypoint { x: w.x, y: w.y + v }, s),
        ('S', v) => (Waypoint { x: w.x, y: w.y - v }, s),
        ('E', v) => (Waypoint { x: w.x + v, y: w.y }, s),
        ('W', v) => (Waypoint { x: w.x - v, y: w.y }, s),

        ('L', v) => rotate_degrees(w, s, -v),
        ('R', v) => rotate_degrees(w, s, v),

        ('F', v) => (w, Ship { x: s.x + (w.x * v), y: s.y + (w.y * v) }),
        _ => panic!("should not happen")
    }
}

// no cos/sin needed in this aoc because every input is 90/180 or 270
fn rotate_degrees(w: Waypoint, s: Ship, angle: i32) -> (Waypoint, Ship) {
    // (negative angle) L becomes an (positive angle) R rotation
    let angle = if angle.is_negative() { 360 - angle.abs() } else { angle };

    let offset_pt = match angle.abs() {
        90 => (w.y, -w.x),
        180 => (-w.x, -w.y),
        270 => (-w.y, w.x),
        _ => panic!("should not happen"),
    };

    (Waypoint { x: offset_pt.0, y: offset_pt.1 }, s)
}

fn main() {
    let input: Vec<(char, i32)> = include_str!("../input.txt")
        .lines()
        .map(|l| {
            let (op, amount) = l.split_at(1);
            (op.chars().next().unwrap(), amount.parse::<i32>().unwrap())
        })
        .collect();

    let mut pos = (0, 0, 0);
    for i in input.clone() {
        pos = part1_travel_one(i, pos);
    }
    println!("Part1 = {}", pos.0.abs() + pos.1.abs());

    let mut waypoint = Waypoint { x: 10, y: 1 };
    let mut ship = Ship { x: 0, y: 0 };
    for i in input {
        let (new_waypoint, new_ship) = part2_travel_one(i, waypoint, ship);
        waypoint = new_waypoint;
        ship = new_ship;
    }
    println!("Part2 = {}", ship.x.abs() + ship.y.abs());
}
