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


fn count_visible_trees(matrix: &Vec<Vec<u8>>) -> usize {
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

fn visibility_range(matrix: &Vec<Vec<u8>>) -> usize {
    let matrix_len = matrix.len();
    let row_len = matrix[0].len();
    let mut result = Vec::new();
    for (i,row) in matrix.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if i == 0 || i == (matrix_len - 1) || j == 0 || j == (row_len - 1) {
                continue;
            }

            let mut size = (0,0,0,0); // top, right, bot, left

            for top in (i + 1)..matrix_len {
                size.0 += 1;
                if matrix[top][j] >= cell {
                    break;
                }
            }


            for right in (j + 1)..row_len {
                size.1 += 1;
                if matrix[i][right] >= cell {
                    break;
                }
            }

            for bot in (0..i).rev() {
                size.2 += 1;
                if matrix[bot][j] >= cell {
                    break;
                }
            }

            for left in (0..j).rev() {
                size.3 += 1;
                if matrix[i][left] >= cell {
                    break;
                }
            }

            result.push((size.0 * size.1 * size.2 * size.3));
        }
    }
    *result.iter().max().unwrap()
}

fn main() {
    let input = std::fs::read_to_string("./inputs/08.in").unwrap();

    let forest = parse(&input);

    println!("part 1: {}", count_visible_trees(&forest));
    println!("part 2: {}", visibility_range(&forest));

}
