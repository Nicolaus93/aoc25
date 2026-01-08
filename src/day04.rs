use std::io::{self};

const POSITIONS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];
const BYTE_OF_AT: u8 = b'@'; // This is a byte literal for '@'

fn explore(matrix: &Vec<Vec<u8>>, i: usize, j: usize, n_rows: usize, n_cols: usize) -> bool {
    let mut count: usize = 0;
    for (x, y) in POSITIONS {
        let row = i as isize + x;
        if row < 0 || row >= n_rows as isize {
            continue;
        }
        let col = j as isize + y;
        if col < 0 || col >= n_cols as isize {
            continue;
        }
        if matrix[row as usize][col as usize] == BYTE_OF_AT {
            count += 1;
            if count >= 4 {
                return false;
            }
        }
    }

    true
}

fn part2(matrix: &mut Vec<Vec<u8>>) -> usize {
    let mut ans = 0;
    let n = matrix.len();
    let m = matrix[0].len();

    let max_it = 50000;
    for _ in 0..max_it {
        let mut idxs: Vec<(usize, usize)> = vec![];
        for i in 0..n {
            for j in 0..m {
                let ch = matrix[i][j];
                if ch == BYTE_OF_AT && explore(&matrix, i, j, n, m) {
                    idxs.push((i, j));
                }
            }
        }
        if idxs.len() == 0 {
            break;
        }
        ans += idxs.len();
        // modify matrix
        for (i, j) in idxs {
            matrix[i][j] = b'.';
        }
    }
    ans
}

pub fn solve04(lines: impl Iterator<Item = io::Result<String>>) -> io::Result<i64> {
    let mut ans = 0;

    // Collect the lines to calculate NUM_ROWS and NUM_COLS
    let matrix: Vec<String> = lines.collect::<Result<Vec<String>, _>>()?;

    // Convert Vec<String> into Vec<Vec<u8>> (Vec of byte vectors)
    let mut matrix_bytes: Vec<Vec<u8>> = matrix
        .into_iter()
        .map(|s| s.into_bytes()) // Convert each String to Vec<u8>
        .collect();

    // check the matrix based on the lines
    let n = matrix_bytes.len();
    let m = matrix_bytes[0].len();
    for i in 0..n {
        for j in 0..m {
            let ch = matrix_bytes[i][j];
            if ch == BYTE_OF_AT && explore(&matrix_bytes, i, j, n, m) {
                ans += 1;
            }
        }
    }

    // part 2
    let part_2 = part2(&mut matrix_bytes);
    println!("Part 2: {}", part_2);

    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![
            Ok(String::from("..@@.@@@@.")),
            Ok(String::from("@@@.@.@.@@")),
            Ok(String::from("@@@@@.@.@@")),
            Ok(String::from("@.@@@@..@.")),
            Ok(String::from("@@.@@@@.@@")),
            Ok(String::from(".@@@@@@@.@")),
            Ok(String::from(".@.@.@.@@@")),
            Ok(String::from("@.@@@.@@@@")),
            Ok(String::from(".@@@@@@@@.")),
            Ok(String::from("@.@.@@@.@.")),
        ];
        let result = solve04(input.into_iter()).unwrap();
        assert_eq!(result, 13);
    }
}
