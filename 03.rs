use std::{collections::HashMap, iter::FromIterator};



fn lookup(needle: char, haystack: &[(char, usize)]) -> usize {
    for &(ch, val) in haystack {
        if ch == needle {
            return val;
        }
    }

    unreachable!();
}


fn gen_items() -> Vec<(char, usize)> {
    let mut items = Vec::with_capacity(52);
    items.extend(('a'..='z').zip(1..=26));
    items.extend(('A'..='Z').zip(27..=52));
    items
}

fn part_two() {
    let items = gen_items();

    let mut input = std::fs::read_to_string("./inputs/03.in").unwrap();

    let mut result = 0;
    let mut chunk = Vec::with_capacity(3);
    for (idx, line) in input.lines().enumerate() {
        if (idx + 1) % 3 == 0 {
            // pushes the third (current) line
            chunk.push(line);
            result += lookup(get_common_char(&chunk), &items);
            chunk.clear();
        } else {
            chunk.push(line);
        }
    }

    println!("{result}");
}

fn get_common_char(lines: &[&str]) -> char {
    for elf_1 in lines[0].chars() {
        for elf_2 in lines[1].chars() {
            for elf_3 in lines[2].chars() {
                if elf_1 == elf_2 && elf_2 == elf_3 {
                    return elf_1;
                }
            }
        }
    }

    unreachable!();
}


fn part_one() {
    let input = std::fs::read_to_string("./inputs/03.in").unwrap();

    let items = gen_items();

    let mut answer: usize = 0;

    for line in input.lines() {
        let mut priority = 0;
        let mut matches = Vec::new();
        let mid = line.len() / 2;
        let split = line.split_at(mid);

        for ch_1 in split.0.chars() {
            for ch_2 in split.1.chars() {
                if ch_1 == ch_2 {
                    matches.push(ch_1);
                }
            }
        }

        matches.sort();
        matches.dedup();

        for ch in matches {
            priority += lookup(ch, &items);
        }

        answer += priority;
    }

    println!("{answer}");
}

fn main() {
    part_two();
}
