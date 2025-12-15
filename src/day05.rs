use std::io;


fn merge_intervals(intervals: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut merged: Vec<(i64, i64)> = Vec::new();
    if intervals.is_empty() {
        return merged;
    }
    
    let mut last_int = intervals[0];
    for int in intervals.iter().skip(1) {
        if int.0 <= last_int.1 {
            last_int.1 = last_int.1.max(int.1);
        } else {
            merged.push(last_int);
            last_int = *int;
        }
    }
    merged.push(last_int);
    merged
}

fn is_in_intervals(n: i64, intervals: &Vec<(i64, i64)>) -> bool {
    for interval in intervals {
        if interval.0 <= n && interval.1 >= n {
            return true;
        }
    }
    false
}

pub fn solve(lines: impl Iterator<Item = io::Result<String>>) -> io::Result<i64> {
    let mut fresh = 0;
    let mut intervals: Vec<(i64, i64)> = Vec::new();
    for line in lines {
        let line = line?;
        if line.contains('-') {
            let (left, right) = line.split_once("-").unwrap();
            let left = left.parse::<i64>().unwrap();
            let right = right.parse::<i64>().unwrap();

            intervals.push((left, right));
        } else if line.len() >= 1 {
            let n = line.parse::<i64>().unwrap();
            if is_in_intervals(n, &intervals) {
                fresh += 1;
            }
        }
    }

    intervals.sort_by_key(|&(a, b)| (a, b));
    let merged = merge_intervals(&intervals);
    let mut part2 = 0;
    for (i, interval) in merged.iter().enumerate() {
        part2 += interval.1 - interval.0 + 1;
        println!("Interval {} {:?}", i, interval);
    }
    println!("Part 2: {}", part2);

    Ok(fresh)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_case_1() {
        // 3-5
        // 10-14
        // 16-20
        // 12-18
        let input = vec![
            Ok(String::from("3-5")),
            Ok(String::from("10-14")),
            Ok(String::from("16-20")),
            Ok(String::from("12-18")),
            Ok(String::from("1")),
            Ok(String::from("5")),
            Ok(String::from("8")),
            Ok(String::from("11")),
            Ok(String::from("17")),
            Ok(String::from("32")),
        ];

        let result = solve(input.into_iter()).unwrap();
        assert_eq!(result, 3);
    }
}
