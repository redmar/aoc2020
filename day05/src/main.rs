fn binlookup_rows(splits: &str) -> u32 {
    let mut v = 0..128;
    for c in splits.chars() {
        let midpoint = (v.end - v.start) / 2;
        if c == 'F' {
            v = v.start..v.start + midpoint;
        } else {
            v = v.end - midpoint..v.end;
        }
    }
    v.start
}

fn binlookup_cols(splits: &str) -> u32 {
    let mut v = 0..8;
    for c in splits.chars() {
        let midpoint = (v.end - v.start) / 2;
        if c == 'L' {
            v = v.start..v.start + midpoint;
        } else {
            v = v.end - midpoint..v.end;
        }
    }
    v.start
}

fn decode_seat(s: &str) -> (u32, u32, u32) {
    let row = binlookup_rows(&s[0..7]);
    let col = binlookup_cols(&s[7..10]);
    let seat_id = row * 8 + col;
    (row, col, seat_id)
}

fn main() {
    let input = include_str!("../input.txt");

    // part1

    let max = input
        .lines()
        .map(|line| decode_seat(line))
        .max_by(|x, y| x.2.cmp(&y.2))
        .map(|(_, _, seat_id)| seat_id)
        .unwrap();

    println!("part1 max seat_id = {}\n", max);

    // part2
    let mut seat_ids: Vec<u32> = input.lines().map(|line| decode_seat(line).2).collect();
    seat_ids.sort();

    for window in seat_ids.windows(3) {
        if window[1] != window[0] + 1 && window[2] != window[0] + 1 {
            println!("part2 seat_id = {}", window[0] + 1);
        }
    }

    println!("part2 (eyeball mode)");
    let mut seats = (0..128)
        .map(|_| (0..8).map(|_| '.').collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    for line in input.lines() {
        let (r, c, _) = decode_seat(line);
        seats[r as usize][c as usize] = '0';
    }
    println!("rows");
    for (row_nr, r) in seats.iter().enumerate() {
        print!("{:0>3} ", row_nr);
        for c in r {
            print!("{}", c);
        }
        print!("\n");
    }
    print!("\n");
}
