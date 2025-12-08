use std::io::{self};

fn count_crossings(old: i32, new: i32) -> i32 {
    let a = old.min(new);
    let b = old.max(new);

    if b - a <= 1 {
        return 0;
    }

    let lower = a + 1;
    let upper = b - 1;

    upper.div_euclid(100) - (lower - 1).div_euclid(100)
}

pub fn solve01(lines: impl Iterator<Item = io::Result<String>>) -> io::Result<i64> {
    let mut pos: i32 = 50;
    let mut stops: i32 = 0;
    let mut passes: i32 = 0;

    for line in lines {
        let line = line?; // <-- now type is correct!
        let line = line.trim();

        let (letter, number_str) = line.split_at(1);
        let n: i32 = number_str
            .parse()
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let old_pos = pos;
        let delta = match letter {
            "L" => -n,
            "R" =>  n,
            _ => return Err(io::Error::new(io::ErrorKind::InvalidData, "bad instruction")),
        };

        passes += count_crossings(old_pos, delta);

        pos = (old_pos + delta).rem_euclid(100);

        if pos == 0 {
            stops += 1;
        }
    }

    Ok((stops + passes) as i64)
}
