fn main() {
    // part 1
    let input = include_str!("../input.txt");
    let mut lines = input.lines();
    let min_depart = lines.next().unwrap().parse::<u32>().unwrap();
    let bus_ids: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .filter_map(|item| item.parse::<u32>().ok())
        .collect();

    let (bus_id_to_take, mins_to_wait) = bus_ids
        .iter()
        .map(|&bus_id| {
            (bus_id, bus_id - min_depart.rem_euclid(bus_id))
        })
        .min_by(|bus_id, other_bus_id| {
            bus_id.1.cmp(&other_bus_id.1)
        }).unwrap();

    println!("Part1 bus to take: {}, you have to wait {} mins", bus_id_to_take, mins_to_wait);
    println!("answer is bus_id * mins_to_wait = {}\n", bus_id_to_take * mins_to_wait);

    // part 2
    let mut lines = input.lines();
    let _first_line = lines.next();
    let bus_ids: Vec<Option<u64>> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|item| item.parse::<u64>().ok())
        .collect();

    let mut stepsize = 1_u64;
    let mut running = 0_u64;
    for (t_offset, busid) in bus_ids.iter().enumerate() {
        if busid.is_none() { continue }

        let busid = busid.unwrap();
        let mut t = running;
        loop {
            if (t + t_offset as u64) % busid == 0 {
                // println!("{} + {} % {} == 0", t, t_offset, busid);
                running = t;
                stepsize *= busid;
                break;
            }
            // println!("{} += {}", t, stepsize);
            t += stepsize;
        }

    }
    println!("Part2 solution = {:?}", running);
}