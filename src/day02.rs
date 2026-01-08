use std::io;

fn is_invalid(s: &str) -> bool {
    let n = s.len();
    if n % 2 != 0 {
        return false;
    }

    let mid = n / 2;
    &s[..mid] == &s[mid..]
}

fn is_really_invalid(s: &str) -> bool {
    let n = s.len();
    for chunk_size in 1..n {
        if all_chunks_equal(s, chunk_size) {
            return true;
        }
    }

    false
}

fn all_chunks_equal(s: &str, n: usize) -> bool {
    s.len() % n == 0
        && s.as_bytes()
            .chunks(n)
            .map(|c| std::str::from_utf8(c).unwrap())
            .all(|chunk| chunk == &s[..n])
}

pub fn solve02(lines: impl Iterator<Item = io::Result<String>>) -> io::Result<i64> {
    let mut invalid_sum: i64 = 0;
    let mut part_1: i64 = 0;
    for line in lines {
        for part in line?.split(',') {
            let (a, b) = part
                .split_once('-')
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "bad range"))?;

            let n1: i64 = a
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            let n2: i64 = b
                .parse()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

            for i in n1..=n2 {
                let n: String = i.to_string();
                if is_invalid(&n) {
                    part_1 += i;
                }
                if is_really_invalid(&n) {
                    // println!("{}-{}: {}", n1, n2, n);
                    invalid_sum += i;
                }
            }
        }
    }
    println!("Part 1: {}", part_1);
    Ok(invalid_sum)
}
