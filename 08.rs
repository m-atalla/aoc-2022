fn parse(input: &str) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    
    for line in input.lines() {
        let mut row = Vec::with_capacity(line.len());
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as u8);
        }
        result.push(row);
    }

    result
}


fn count_visible_trees(matrix: Vec<Vec<u8>>) -> usize {
    let mut count = 0;

    // is such an optimization possible with `LICM`?
    // requires
    let matrix_len = matrix.len();
    let row_len = matrix[0].len();

    for (i, row) in matrix.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if i == 0 || (i + 1) == matrix_len {
                count += 1;
                continue;
            }

            if j == 0 || (j + 1) == row_len {
                count += 1;
                continue;
            }

            // directions: above, below, left, right
            let mut visible: Vec<bool>= Vec::with_capacity(4);

            // there is probably a _smart_ way to do this but I couldnt bother to do it..
            let mut v = true;

            for l in (j+1)..row_len {
                if matrix[i][l] >= cell {
                    v = false;
                    break;
                }
            }

            visible.push(v);

            v = true;

            for r in 0..j {
                if matrix[i][r] >= cell {
                    v = false;
                    break;
                }
            }

            visible.push(v);

            v = true;

            for d in (i + 1)..matrix_len {
                if matrix[d][j] >= cell {
                    v = false;
                    break;
                }
            }

            visible.push(v);

            v = true;

            for u in 0..i {
                if matrix[u][j] >= cell {
                    v = false;
                    break;
                }
            }

            visible.push(v);

            if !visible.is_empty() && visible.iter().any(|&visible| visible) {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input = std::fs::read_to_string("./inputs/08.in").unwrap();

    let forest = parse(&input);

    println!("part 1: {}", count_visible_trees(forest));

}
