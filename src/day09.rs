use itertools::Itertools;
use std::{fmt, io};

#[derive(Debug)]
struct Point2D {
    x: i64,
    y: i64,
}

// Implement Display for Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point2D {

    // Method to create a Point2D from a string like "1,2"
    fn from_string(s: &str) -> Result<Self, &'static str> {
        let coords: Vec<&str> = s.split(',').collect();
        if coords.len() != 2 {
            return Err("Input string must be in the format 'x,y,z'");
        }
        let x = coords[0].trim().parse::<i64>().map_err(|_| "Failed to parse x")?;
        let y = coords[1].trim().parse::<i64>().map_err(|_| "Failed to parse y")?;
        Ok(Point2D { x, y})
    }

}


pub fn solve(lines: impl Iterator<Item = io::Result<String>>) -> io::Result<i64> {

    let points: Vec<Point2D> = lines
        .filter_map(|line| {
            // Filter out any lines that fail to read
            let line = line.ok()?;
            // Parse the line into a Point3D
            Point2D::from_string(&line).ok()
        })
        .collect();

    let areas: Vec<i64> = points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1)
        })
        .collect();

    Ok(*areas.iter().max().unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input_str = "
            7,1
            11,1
            11,7
            9,7
            9,5
            2,5
            2,3
            7,3";
        let input: Vec<Result<String, _>> = input_str
            .lines()
            .map(|line| Ok(line.trim().to_string())) // Trim each line
            .filter(|line| !line.as_ref().unwrap().is_empty()) // Skip empty lines
            .collect();

        let result = solve(input.into_iter()).unwrap();
        assert_eq!(result, 50);
    }
}
