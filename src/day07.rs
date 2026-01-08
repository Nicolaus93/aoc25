use std::collections::{HashMap, HashSet};
use std::io;

fn explore(
    mut i: usize,
    j: usize,
    matrix: &Vec<Vec<char>>,
    splits: &mut HashSet<(usize, usize)>,
    visited: &mut HashSet<(usize, usize)>,
) {
    let n = matrix.len();
    let m = matrix[0].len();
    while i < n {
        visited.insert((i, j));
        if matrix[i][j] == '^' {
            splits.insert((i, j));
            if j - 1 > 0 && !visited.contains(&(i, j - 1)) {
                explore(i, j - 1, matrix, splits, visited);
            }
            if j + 1 < m && !visited.contains(&(i, j + 1)) {
                explore(i, j + 1, matrix, splits, visited);
            }
            return;
        }
        i += 1;
    }
}

fn get_all_paths(
    mut i: usize,
    j: usize,
    matrix: &Vec<Vec<char>>,
    solutions: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    if solutions.contains_key(&(i, j)) {
        return solutions[&(i, j)];
    }
    let n = matrix.len();
    let m = matrix[0].len();
    while i < n {
        if matrix[i][j] == '^' {
            let mut res: i64 = 0;
            if j >= 1 {
                let partial_sol = get_all_paths(i, j - 1, &matrix, solutions);
                solutions.insert((i, j - 1), partial_sol);
                res += partial_sol;
            }
            if j < m - 1 {
                let partial_sol = get_all_paths(i, j + 1, &matrix, solutions);
                solutions.insert((i, j + 1), partial_sol);
                res += partial_sol;
            }
            // solutions.insert((i, j), res);
            return res;
        }
        i += 1;
    }
    solutions.insert((i, j), 1);
    1
}

pub fn solve(lines: impl Iterator<Item = io::Result<String>>) -> io::Result<i64> {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let line = line?;
        matrix.push(line.chars().collect());
    }

    let n = matrix.len();
    let m = matrix[0].len();
    let mut splits: HashSet<(usize, usize)> = HashSet::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == 'S' {
                visited.insert((i, j));
                explore(i, j, &mut matrix, &mut splits, &mut visited);
                let mut solutions: HashMap<(usize, usize), i64> = HashMap::new();
                let part2 = get_all_paths(i, j, &matrix, &mut solutions);
                println!("Part 2: {}", part2);
            }
        }
    }
    Ok(splits.len() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input_str = "
        .......S.......
        ...............
        .......^.......
        ...............
        ......^.^......
        ...............
        .....^.^.^.....
        ...............
        ....^.^...^....
        ...............
        ...^.^...^.^...
        ...............
        ..^...^.....^..
        ...............
        .^.^.^.^.^...^.
        ...............";
        let input: Vec<Result<String, _>> = input_str
            .lines()
            .map(|line| Ok(line.trim().to_string())) // Trim each line
            .filter(|line| !line.as_ref().unwrap().is_empty()) // Skip empty lines
            .collect();

        let result = solve(input.into_iter()).unwrap();
        assert_eq!(result, 21);
    }
}
