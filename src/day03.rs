use std::cmp::max;
use std::io;

fn part_1(bytes: &[u8]) -> i64 {
    let n = bytes.len();
    let mut m = 0;
    for i in 0..n {
        let d1 = (bytes[i] - b'0') as i64;

        for j in i + 1..n {
            let d2 = (bytes[j] - b'0') as i64;

            let num = d1 * 10 + d2; // two-digit number
            m = max(m, num);
        }
    }
    m
}

const DEPTH: usize = 12;

// Recursive function to find the maximum number by either including or skipping digits
fn part_2(bytes: &[u8]) -> i64 {
    let n = bytes.len();
    let mut max_value: i64 = 0;
    let mut remaining_digits = DEPTH;
    let mut index = 0;

    // Greedily select digits, considering from left to right
    while remaining_digits > 0 {
        let mut max_digit = 0;
        let mut max_digit_index = index;

        // Search for the max digit within the valid range
        for i in index..=n - remaining_digits {
            let digit = (bytes[i] - b'0') as i64;
            if digit > max_digit {
                max_digit = digit;
                max_digit_index = i;
            }
        }

        // Build the number by appending the max digit found
        max_value = max_value * 10 + max_digit;

        // Update index to continue from the next position after the selected digit
        index = max_digit_index + 1;
        remaining_digits -= 1;
    }
    max_value
}

pub fn solve03(lines: impl Iterator<Item = io::Result<String>>) -> io::Result<i64> {
    let mut total: i64 = 0;

    for (i, line) in lines.into_iter().enumerate() {
        println!("Line {}", i);
        let line = line?;
        let bytes = line.as_bytes();
        println!("part 1 {}", part_1(&bytes));
        let m = part_2(&bytes);

        total += m; // Add the result for the current line
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_case_1() {
        // Given the input "987654321111111", the largest two-digit number is 98
        let input = b"987654321111111"; // byte literal for the input string
        let result = part_1(input);
        assert_eq!(result, 98); // The expected result is 98
    }

    #[test]
    fn test_part_1_case_2() {
        let input = b"811111111111119"; // byte literal for the input string
        let result = part_1(input);
        assert_eq!(result, 89); // The expected result is 98
    }

    #[test]
    fn test_part_1_case_3() {
        let input = b"234234234234278"; // byte literal for the input string
        let result = part_1(input);
        assert_eq!(result, 78); // The expected result is 98
    }

    #[test]
    fn test_part_1_case_4() {
        let input = b"818181911112111"; // byte literal for the input string
        let result = part_1(input);
        assert_eq!(result, 92); // The expected result is 98
    }

    #[test]
    fn test_part2_case_1() {
        let input = vec![Ok(String::from("987654321111111"))];
        let result = solve03(input.into_iter()).unwrap();
        assert_eq!(result, 987654321111);
    }

    #[test]
    fn test_part2_case_2() {
        let input = vec![Ok(String::from("811111111111119"))];
        let result = solve03(input.into_iter()).unwrap();
        assert_eq!(result, 811111111119);
    }

    #[test]
    fn test_part2_case_3() {
        let input = vec![Ok(String::from("234234234234278"))];
        let result = solve03(input.into_iter()).unwrap();
        assert_eq!(result, 434234234278);
    }
}
