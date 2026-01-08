use itertools::Itertools;
use std::collections::BTreeSet;
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
        let x = coords[0]
            .trim()
            .parse::<i64>()
            .map_err(|_| "Failed to parse x")?;
        let y = coords[1]
            .trim()
            .parse::<i64>()
            .map_err(|_| "Failed to parse y")?;
        Ok(Point2D { x, y })
    }
}

pub fn solve_part_1(lines: impl Iterator<Item = io::Result<String>>) -> io::Result<i64> {
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
        .map(|(a, b)| ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1))
        .collect();

    Ok(*areas.iter().max().unwrap())
}

/// Inclusive rectangle area in "tiles" (matches your examples):
/// width = |x2-x1| + 1, height = |y2-y1| + 1
fn rect_area_tiles(a: &Point2D, b: &Point2D) -> i64 {
    let w = (a.x - b.x).abs() + 1;
    let h = (a.y - b.y).abs() + 1;
    w * h
}

/// Standard even-odd point-in-polygon test for an orthogonal polygon.
/// Works on a point p in continuous plane. Boundary is treated as inside.
/// Polygon vertices must form a simple closed loop in order.
fn point_in_poly_even_odd(p: (f64, f64), poly: &[Point2D]) -> bool {
    let (px, py) = p;
    let n = poly.len();
    let mut inside = false;

    for i in 0..n {
        let a = &poly[i];
        let b = &poly[(i + 1) % n];

        let (x1, y1) = (a.x as f64, a.y as f64);
        let (x2, y2) = (b.x as f64, b.y as f64);

        // Boundary check for axis-aligned edges
        if y1 == y2 {
            // horizontal
            if (py == y1) && (px >= x1.min(x2)) && (px <= x1.max(x2)) {
                return true;
            }
        } else if x1 == x2 {
            // vertical
            if (px == x1) && (py >= y1.min(y2)) && (py <= y1.max(y2)) {
                return true;
            }
        }

        // Ray cast to +inf in x direction: count crossings
        // Consider edges that straddle py
        let cond = (y1 > py) != (y2 > py);
        if cond {
            let x_int = x1 + (py - y1) * (x2 - x1) / (y2 - y1);
            if x_int > px {
                inside = !inside;
            }
        }
    }

    inside
}

/// Build a compressed grid from polygon vertex coordinates:
/// X coords are unique vertex x's; Y coords are unique vertex y's.
/// Then determine which cells are inside the polygon by testing cell centers.
/// Finally, build a weighted prefix sum over cell areas (in continuous area units).
fn build_allowed_prefix(poly: &[Point2D]) -> (Vec<i64>, Vec<i64>, Vec<Vec<i128>>) {
    let mut xs_set = BTreeSet::<i64>::new();
    let mut ys_set = BTreeSet::<i64>::new();
    for p in poly {
        xs_set.insert(p.x);
        ys_set.insert(p.y);
    }
    let xs: Vec<i64> = xs_set.into_iter().collect();
    let ys: Vec<i64> = ys_set.into_iter().collect();

    // cells exist between consecutive coordinate lines
    let w = xs.len().saturating_sub(1);
    let h = ys.len().saturating_sub(1);

    // A[y][x] = area of cell if inside else 0 (use i128 to avoid overflow)
    let mut a = vec![vec![0i128; w]; h];

    for yi in 0..h {
        let y0 = ys[yi] as f64;
        let y1 = ys[yi + 1] as f64;
        let cy = (y0 + y1) / 2.0;
        let cell_h = (ys[yi + 1] - ys[yi]) as i128;

        for xi in 0..w {
            let x0 = xs[xi] as f64;
            let x1 = xs[xi + 1] as f64;
            let cx = (x0 + x1) / 2.0;
            let cell_w = (xs[xi + 1] - xs[xi]) as i128;

            if point_in_poly_even_odd((cx, cy), poly) {
                a[yi][xi] = cell_w * cell_h;
            }
        }
    }

    // Prefix sum P with padding: (h+1) x (w+1)
    let mut p = vec![vec![0i128; w + 1]; h + 1];
    for y in 0..h {
        for x in 0..w {
            p[y + 1][x + 1] = a[y][x] + p[y][x + 1] + p[y + 1][x] - p[y][x];
        }
    }

    (xs, ys, p)
}

/// Query sum of allowed area (continuous) for a rectangle aligned to coordinate indices:
/// x in [x1, x2), y in [y1, y2) in the compressed cell index space (half-open in cells).
fn rect_sum(p: &[Vec<i128>], x1: usize, y1: usize, x2: usize, y2: usize) -> i128 {
    // p is (h+1)x(w+1)
    p[y2][x2] - p[y1][x2] - p[y2][x1] + p[y1][x1]
}

pub fn solve(lines: impl Iterator<Item = io::Result<String>>) -> io::Result<i64> {
    let points: Vec<Point2D> = lines
        .filter_map(|line| {
            let line = line.ok()?;
            Point2D::from_string(&line).ok()
        })
        .collect();

    // Build compressed-grid prefix sum of allowed interior (continuous).
    let (xs, ys, pref) = build_allowed_prefix(&points);

    let mut best: i64 = 0;

    // Enumerate opposite-corner red pairs (O(R^2))
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let a = &points[i];
            let b = &points[j];

            if a.x == b.x || a.y == b.y {
                continue; // degenerate rectangle (line)
            }

            // Rectangle corners in tile coords (inclusive tiles):
            // Use min/max for area in tiles
            let min_x = a.x.min(b.x);
            let max_x = a.x.max(b.x);
            let min_y = a.y.min(b.y);
            let max_y = a.y.max(b.y);

            // Quick upper bound pruning
            let area_tiles = rect_area_tiles(a, b);
            if area_tiles <= best {
                continue;
            }

            // For the continuous prefix grid:
            // we want the area of the rectangle [min_x, max_x] x [min_y, max_y]
            // in coordinate space. This requires these coordinates to be in xs/ys.
            let ix1 = xs.binary_search(&min_x).expect("coord not in lines");
            let ix2 = xs.binary_search(&max_x).expect("coord not in lines");
            let iy1 = ys.binary_search(&min_y).expect("coord not in lines");
            let iy2 = ys.binary_search(&max_y).expect("coord not in lines");

            // Cells are between lines, so rectangle covering [min_x,max_x] spans cell indices [ix1,ix2)
            // BUT since max_x is a line, to include the region up to max_x, we use ix2 (already the larger index)
            // same for y.
            let allowed_area = rect_sum(&pref, ix1, iy1, ix2, iy2);

            // Rectangle continuous area:
            let rect_area_cont = (max_x - min_x) as i128 * (max_y - min_y) as i128;

            // If fully inside (continuous), accept.
            // (If you need exact *tile* coverage, this is the spot to adjust with +1 / scaling.)
            if allowed_area == rect_area_cont {
                best = area_tiles;
            }
        }
    }

    Ok(best)
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
        assert_eq!(result, 24);
    }
}
