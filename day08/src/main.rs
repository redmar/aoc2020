#[derive(Debug, Copy, Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

#[derive(Debug, PartialEq)]
enum Mode {
    Part1,
    Part2,
}

impl Instruction {
    fn parse_instruction(line: &str) -> Instruction {
        let mut itr = line.split(' ');
        let instruction = itr.next();
        let nr = itr.next().unwrap().parse::<i32>().unwrap();
        match instruction {
            Some("nop") => Instruction::Nop(nr),
            Some("acc") => Instruction::Acc(nr),
            Some("jmp") => Instruction::Jmp(nr),
            Some(_) | None => panic!("unhandled instruction"),
        }
    }
}

fn run_bootcode(
    program: &Vec<Instruction>,
    mode: Mode,
    overlay_instruction: Option<(usize, Instruction)>,
) -> Option<i32> {
    let mut ic: i32 = 0;
    let mut acc = 0;
    let mut offset: i32 = 0;
    // let mut prev_ic = 0;
    let mut visited: Vec<bool> = Vec::new();
    for _ in 0..program.len() {
        visited.push(false);
    }

    loop {
        // prev_ic = ic;
        let offset_until_wrap = (program.len() as i32 - 1) - ic;
        if offset > offset_until_wrap {
            ic = 0 + offset - offset_until_wrap;
        } else if ic + offset < 0 {
            ic = (program.len() as i32 - 1) + (ic + offset);
        } else {
            ic += offset;
        }

        if visited[ic as usize] {
            if mode == Mode::Part1 {
                // println!("visited {} twice! acc = {}", prev_ic, acc);
                return Some(acc);
            } else {
                return None;
            }
        } else {
            visited[ic as usize] = true;
        }

        let current_instruction =
            if let Some((overlay_idx, overlay_instruction)) = overlay_instruction {
                if overlay_idx == (ic as usize) {
                    overlay_instruction
                } else {
                    program[ic as usize]
                }
            } else {
                program[ic as usize]
            };

        // println!("{} = {:?} \t| {}", ic, current_instruction, acc);
        match current_instruction {
            Instruction::Nop(_) => offset = 1,
            Instruction::Acc(inc) => {
                acc += inc;
                offset = 1
            }
            Instruction::Jmp(inc) => offset = inc,
        };

        if mode == Mode::Part2 && ic as usize == program.len() - 1 {
            // println!("last instruction executed! acc = {}", acc);
            return Some(acc);
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let program = input
        .lines()
        .map(Instruction::parse_instruction)
        .collect::<Vec<Instruction>>();

    println!("part1 = {:?}", run_bootcode(&program, Mode::Part1, None));

    for (idx, mutate_instruction) in program.iter().enumerate() {
        let overlay_instruction = match mutate_instruction {
            Instruction::Nop(x) => Some((idx, Instruction::Jmp(*x))),
            Instruction::Acc(_inc) => None,
            Instruction::Jmp(x) => Some((idx, Instruction::Nop(*x))),
        };

        if let Some(acc) = run_bootcode(&program, Mode::Part2, overlay_instruction) {
            println!("part2 = {:?}", acc);
            break;
        }
    }
}
