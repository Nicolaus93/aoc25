use std::io;

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Prod,
}

fn part2(matrix: &Vec<Vec<char>>) -> i64 {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut res: i64 = 0;
    let mut op: Op = Op::Add;
    let mut block_res: i64 = 0;
    for col in 0..cols {
        if matrix[rows - 1][col] == '*' {
            res += block_res;
            // println!("partial res: {}", block_res);
            block_res = 1;
            op = Op::Prod;
        } else if matrix[rows - 1][col] == '+' {
            res += block_res;
            // println!("partial res: {}", block_res);
            block_res = 0;
            op = Op::Add;
        }
        let number: i64 = matrix
            .iter()
            .take(rows - 1) // skip last row
            .map(|row| row[col])
            .filter_map(|c| c.to_digit(10)) // char → digit
            .fold(0_i64, |acc, d| acc * 10 + d as i64);
        if number == 0 {
            continue;
        }

        block_res = match op {
            Op::Add => block_res + number,
            Op::Prod => block_res * number,
        }
    }
    res += block_res;
    // println!("partial res: {}", block_res);
    res
}

pub fn solve(lines: impl Iterator<Item = io::Result<String>>) -> io::Result<i64> {
    let mut matrix: Vec<Vec<i64>> = Vec::new();
    let mut ans = 0;
    let mut matrix2: Vec<Vec<char>> = Vec::new();

    for raw_line in lines {
        let line_str = raw_line?;
        // collect all chars for part2
        matrix2.push(line_str.chars().collect());

        let line = line_str.trim();
        if line.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = line.split_whitespace().collect();

        // If every token is an int -> matrix row
        if tokens.iter().all(|t| t.parse::<i64>().is_ok()) {
            let row: Vec<i64> = tokens.iter().map(|t| t.parse::<i64>().unwrap()).collect();
            matrix.push(row);
        } else {
            for (i, op) in tokens.iter().enumerate() {
                let col: Vec<i64> = matrix.iter().map(|row| row[i]).collect();
                let s: i64 = match *op {
                    "+" => col.iter().sum(),
                    "*" => col.iter().product(),
                    _ => 0, // or return Err(...)
                };
                ans += s;
            }
        }
    }

    println!("part1: {}", ans);
    let ans2 = part2(&matrix2);
    Ok(ans2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![
            Ok(String::from("123 328  51 64 ")),
            Ok(String::from(" 45 64  387 23 ")),
            Ok(String::from("  6 98  215 314")),
            Ok(String::from("*   +   *   +  ")),
        ];

        let result = solve(input.into_iter()).unwrap();
        assert_eq!(result, 3263827);
    }
}
