fn part1(input: &Vec<u64>, preamble_size: usize) -> Option<u64> {
    let start = std::time::Instant::now();
    let window_size = preamble_size;
    let mut min = 0;
    let mut max = min + window_size;
    let mut found_needle;
    let mut invalid_number: Option<u64> = None;
    for needle in input[(window_size + 1)..input.len()].iter() {
        found_needle = false;
        'needle_search: for i in input[min..=max].iter() {
            for j in input[min..=max].iter() {
                if i + j == *needle {
                    found_needle = true;
                    min += 1;
                    max = min + window_size;

                    break 'needle_search;
                }
            }
        }
        if found_needle == false {
            println!("part1 = no sum found for {}", needle);
            invalid_number = Some(needle.clone());
            break;
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}\n", duration);
    invalid_number
}

fn part2(input: &Vec<u64>, invalid_number: u64) -> Option<(u64, u64)> {
    let start = std::time::Instant::now();
    let mut window_size = 2;
    let mut min = 0;
    let mut max = min + window_size - 1;
    let min_max: Option<(u64, u64)>;

    'outer: loop {
        if input[min..=max].iter().sum::<u64>() == invalid_number {
            let min_of_seq = *input[min..=max].iter().min().unwrap();
            let max_of_seq = *input[min..=max].iter().max().unwrap();
            println!("min: {:?}", min_of_seq);
            println!("max: {:?}", max_of_seq);
            min_max = Some((min_of_seq, max_of_seq));
            break 'outer;
        }

        min += 1;
        max = min + window_size - 1;

        if max > input.len() - 1 {
            window_size += 1;
            min = 0;
            max = min + window_size - 1;
        }
    }
    let duration = start.elapsed();
    println!("Time elapsed: {:?}\n", duration);

    min_max
}

fn main() {
    let input: Vec<u64> = include_str!("../input.txt")
        .lines()
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    let invalid_number = part1(&input, 25).unwrap(); // preamble_size = 5 for test.txt

    match part2(&input, invalid_number) {
        Some((min, max)) => println!("part2 = {:?} ", min + max),
        None => panic!("ohnoes, no answer found for part1? you made a mistake i'm afraid!"),
    }
}
