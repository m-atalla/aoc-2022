fn gen_ranges(len: usize) -> Vec<std::ops::Range<usize>>{
    let mut ranges = Vec::new();

    let mut idx = 0;

    for _ in 0..len {
        ranges.push(idx..(idx+3));

        idx += 4;
    }
    
    ranges
}

fn parse_stacks(stack_str: &str) -> Vec<Vec<String>> {
    let mut stacks: Vec<Vec<String>> = Vec::new();

    for _ in stack_str.lines().last().unwrap().split("  ") {
        stacks.push(Vec::new());
    }


    let ranges = gen_ranges(stacks.len());

    for line in stack_str.lines().rev().skip(1) {
        for (idx, range) in ranges.iter().enumerate() {

            let payload = line[range.start..range.end].to_string();

            // skip empty cells
            if payload == "   " {
                continue;
            }

            // I think there is a simpler way to do the range here
            // not sure..
            stacks[idx].push(payload.clone());
        }
    }

    stacks
}

fn exec_inst<F>(inst: &str, stacks: &mut Vec<Vec<String>>, exec: F)
where
    F: Fn(&mut Vec<Vec<String>>, (usize, usize, usize))
{
    for line in inst.lines() {
        let num_inst: Vec<usize> = line.split(" ")
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();


        // `-1` to match array indices
        let instructions = (num_inst[0], num_inst[1] -1, num_inst[2] - 1);

        exec(stacks, instructions);
    }
}

fn part_one_movement(stacks: &mut Vec<Vec<String>>, (count, from, to): (usize, usize, usize)) {
    for _ in 0..count {
        let payload = stacks[from].pop().unwrap();
        stacks[to].push(payload);
    }
}


/// uses a temp stack to keep movement **in-order**.
fn part_two_movement(stacks: &mut Vec<Vec<String>>, (count, from, to): (usize, usize, usize)) {
    let mut temp = Vec::with_capacity(count);

    for _ in 0..count {
        let payload = stacks[from].pop().unwrap();
        temp.push(payload);
    }

    for _ in 0..temp.len() {
        let payload = temp.pop().unwrap();
        stacks[to].push(payload);
    }
}

fn solve<F>(exec_fn: F) 
where
    F: Fn(&mut Vec<Vec<String>>, (usize, usize, usize))
{
    let input = std::fs::read_to_string("./inputs/05.in").unwrap();

    let (stacks, inst) = input.split_once("\n\n").unwrap();

    let mut stacks = parse_stacks(stacks);

    exec_inst(inst, &mut stacks, exec_fn);

    for stack in stacks {
        let ch: String = stack.last().unwrap().chars().filter(|c| c.is_alphabetic()).collect();
        print!("{}", ch);
    }

    print!("\n");
}

fn main() {
    println!("part 1: ");
    solve(part_one_movement);


    println!("part 2: ");
    solve(part_two_movement);
}
