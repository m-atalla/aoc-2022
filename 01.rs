use std::fs;

fn main() {
    // part_one();
    part_two()
}

fn part_two() {
    let mut elves_inv: Vec<u64> = vec![0];
    let input = fs::read_to_string("./inputs/01.in").unwrap();
    let mut curr_elf = 0;
    input.lines().for_each(|line| {
        // move to the next elf 
        if line.is_empty() {
            curr_elf += 1;
            elves_inv.push(0);
            return;
        }

        elves_inv[curr_elf] += line.parse::<u64>().unwrap();
    });

    elves_inv.sort();

    let max_three_sum: u64 = elves_inv.iter().rev().take(3).sum(); // lol
    println!("{max_three_sum}");
}

fn part_one() {
    let mut elves_inv: Vec<u64> = vec![0];

    let input = fs::read_to_string("./inputs/01.in").unwrap();
    let mut curr_elf = 0;
    let mut max_elf_idx = 0;

    input.lines().for_each(|line| {
        // move to the next elf 
        if line.is_empty() {
            curr_elf += 1;
            elves_inv.push(0);
            return;
        }

        elves_inv[curr_elf] += line.parse::<u64>().unwrap();

        if elves_inv[curr_elf] > elves_inv[max_elf_idx] {
            max_elf_idx = curr_elf;
        }
    });


    println!("{}", elves_inv[max_elf_idx]);
}


