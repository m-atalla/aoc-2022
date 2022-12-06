fn solve(buffer_size: usize) {
    let input = std::fs::read_to_string("./inputs/06.in").unwrap();

    let mut buf = "".to_string();

    for (idx, ch) in input.chars().enumerate() {


        for (buf_idx, buf_ch) in buf.clone().chars().enumerate() {
            if ch == buf_ch {
                buf = buf[(buf_idx + 1)..].to_string();
                break;
            }
        }

        buf.push(ch);

        if buf.len() == buffer_size {
            println!("idx: {}", idx + 1);
            break;
        }
    }
}

fn main() {
    println!("Part one:");
    solve(4);

    println!("Part two:");
    solve(14);
}
