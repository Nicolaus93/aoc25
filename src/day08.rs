use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io;

#[derive(Debug)]
struct Point3D {
    x: i64,
    y: i64,
    z: i64,
}

// Implement Display for Point3D
impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Point3D {
    // Method to create a Point3D from a string like "1,2,3"
    fn from_string(s: &str) -> Result<Self, &'static str> {
        let coords: Vec<&str> = s.split(',').collect();
        if coords.len() != 3 {
            return Err("Input string must be in the format 'x,y,z'");
        }
        let x = coords[0]
            .trim()
            .parse::<i64>()
            .map_err(|_| "Failed to parse x")?;
        let y = coords[1]
            .trim()
            .parse::<i64>()
            .map_err(|_| "Failed to parse y")?;
        let z = coords[2]
            .trim()
            .parse::<i64>()
            .map_err(|_| "Failed to parse z")?;
        Ok(Point3D { x, y, z })
    }

    // Method to calculate squared distance to another Point3D
    fn sqd_dist(&self, other: &Point3D) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx.pow(2) + dy.pow(2) + dz.pow(2)
    }
}

pub fn solve(lines: impl Iterator<Item = io::Result<String>>) -> io::Result<i64> {
    let points: Vec<Point3D> = lines
        .filter_map(|line| {
            // Filter out any lines that fail to read
            let line = line.ok()?;
            // Parse the line into a Point3D
            Point3D::from_string(&line).ok()
        })
        .collect();

    // store the circuit for each point
    let mut circuit_map: Vec<usize> = (0..points.len()).collect();

    // store each pairwise distance
    let mut distance_map: HashMap<i64, (usize, usize)> = HashMap::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dist = points[i].sqd_dist(&points[j]);
            distance_map.insert(dist, (i, j));
        }
    }

    // Sort the distances in ascending order
    let mut sorted_distances: Vec<_> = distance_map.keys().collect();
    sorted_distances.sort();

    let n = 10000;
    for idx in 0..n {
        let dist = sorted_distances[idx];
        let &(i, j) = distance_map.get(dist).unwrap();
        // Get the circuit indices of the two points
        let circuit_i = circuit_map[i];
        let circuit_j = circuit_map[j];
        // Merge the circuits: update all points in circuit_j to be in circuit_i
        for circuit_idx in circuit_map.iter_mut() {
            if *circuit_idx == circuit_j {
                *circuit_idx = circuit_i;
            }
        }

        let unique_idxs: HashSet<usize> = circuit_map.iter().copied().collect();
        let num_unique_circuits = unique_idxs.len();
        if num_unique_circuits == 1 {
            println!(
                "{}, {} => {}",
                points[i],
                points[j],
                points[i].x * points[j].x
            );
            break;
        }
    }

    // Print the final state of circuits
    let mut circuit_counts: HashMap<usize, usize> = HashMap::new();
    for &circuit in &circuit_map {
        *circuit_counts.entry(circuit).or_insert(0) += 1;
    }

    println!("Final circuit state:");
    for (&circuit, &count) in circuit_counts.iter() {
        println!("Circuit {} has {} points", circuit, count);
    }

    // Collect the counts into a vector and sort it in descending order
    let mut counts: Vec<usize> = circuit_counts.values().cloned().collect();
    counts.sort_by(|a, b| b.cmp(a)); // Sort in descending order

    // Take the top 3 counts, convert to i64, and multiply them together
    let product: i64 = counts
        .iter()
        .take(3)
        .fold(1i64, |acc, &count| acc * count as i64);

    // Print the sorted counts and the product
    println!("Sorted circuit counts: {:?}", counts);
    println!("Product of the top 3 largest circuits: {}", product);
    println!();

    Ok(product)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input_str = "
            162,817,812
            57,618,57
            906,360,560
            592,479,940
            352,342,300
            466,668,158
            542,29,236
            431,825,988
            739,650,466
            52,470,668
            216,146,977
            819,987,18
            117,168,530
            805,96,715
            346,949,466
            970,615,88
            941,993,340
            862,61,35
            984,92,344
            425,690,689";
        let input: Vec<Result<String, _>> = input_str
            .lines()
            .map(|line| Ok(line.trim().to_string())) // Trim each line
            .filter(|line| !line.as_ref().unwrap().is_empty()) // Skip empty lines
            .collect();

        let result = solve(input.into_iter()).unwrap();
        assert_eq!(result, 40);
    }
}
