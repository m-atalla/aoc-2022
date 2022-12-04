use std::ops::RangeInclusive;



#[inline]
fn parse(n: &str) -> usize {
    n.parse::<usize>().unwrap()
}


fn range_from((n1, n2): (&str, &str)) -> RangeInclusive<usize> {
    RangeInclusive::new(parse(n1), parse(n2))
}


fn is_full_contained(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    if a.start() >= b.start() && a.end() <= b.end() {
        true
    } else if b.start() >= a.start() && b.end() <= a.end() {
        true
    } else {
        false
    }
}

fn is_partially_contained(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool { 
    // 1 2 3
    //     3 4 5
    if b.contains(a.start()) || b.contains(a.end()) {
        true
    } else if a.contains(b.start()) || a.contains(b.end()) {
        true
    } else {
        is_full_contained(a, b)
    }
}

fn solve<P>(predicate: P)
where
    P: Fn(&RangeInclusive<usize>, &RangeInclusive<usize>) -> bool 
{
    let input = std::fs::read_to_string("./inputs/04.in").unwrap();

    let result = input.lines().map(|line | {
        let (a, b) = line.split_once(',').unwrap();

        let (a_parts, b_parts) = (a.split_once('-').unwrap(), b.split_once('-').unwrap());


        let (a_range, b_range) = (range_from(a_parts), range_from(b_parts));

        (a_range, b_range)
    })
    .filter(|(a, b)| predicate(a, b))
    .count();


    println!("{}", result);
}

fn main() {
    println!("part 1:");
    solve(is_full_contained);
    println!("part 2:");
    solve(is_partially_contained);
}
