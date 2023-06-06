use std::collections::HashSet;

type IntPoint = (i64, i64);

pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
    if coordinates.len()<3{
        return true;
    }
    if let Some(p1) = coordinates.get(0){
        if let Some(p2) = coordinates.get(1){
            let slope1 = (p2[1] - p1[1]) as f32 /  (p2[0] - p1[0]) as f32;
            for c in &coordinates[2..] {
                let slope2 = (c[1] - p1[1]) as f32 /  (c[0] - p1[0]) as f32;
                if slope2 != slope1 && (slope1.is_finite() || slope2.is_finite()){
                    return false;
                } 
            }
        }
    }
    true
}

/// Calculate the slope between two points.
/// We return a float to not lose precision, which is NOT `Eq` or `Hash`.
fn slope(p1: &IntPoint, p2: &IntPoint) -> f64 {
    (p2.1 - p1.1) as f64 / (p2.0 - p1.0) as f64
}

/// Calculate the y-intersect of a line given a point and the slope.
fn y_intersect(p1: &IntPoint, slope: f64) -> f64 {
    p1.1 as f64 - (slope * p1.0 as f64)
}

/// Number of points on the same line.
/// <https://buttondown.email/cassidoo/archive/9753/>
pub fn max_points_on_line(points: &[IntPoint]) -> usize {
    // No point provided.
    if points.is_empty() {
        return 0;
    }
    // One point provided.
    if points.len() == 1 {
        return 1;
    }
    // Calculate slopes and y intersects for all combination of points, just keeping the point indices.
    let mut slopes: Vec<((f64, f64), usize, usize)> = points
        .iter()
        .enumerate()
        .flat_map(|(ix1, p1)| {
            points
                .iter()
                .enumerate()
                .skip(ix1 + 1)
                .map(move |(ix2, p2)| {
                    let slope = slope(p1, p2);
                    let y = y_intersect(p1, slope);
                    ((slope, y), ix1, ix2)
                })
        })
        .collect();
    // Sort vec so that all points on the same line are next to each other.
    slopes.sort_unstable_by(|(a, _, _), (b, _, _)| a.partial_cmp(b).unwrap());
    // Put all indices for the same line in a Hashset (we cannot use f64 directly as keys).
    let mut max = 2;
    let mut line = HashSet::new();
    let mut it = slopes.into_iter();
    // First point, safe to get since if there's less that one point we returned earlier.
    let mut fst = it.next().unwrap();
    line.insert(fst.1);
    line.insert(fst.2);
    for f in it {
        // Same line as before.
        if f.0 == fst.0 {
            // Insert point.
            line.insert(f.1);
            line.insert(f.2);
            // Number of points
            max = max.max(line.len());
        // New line.
        } else {
            fst = f;
            line.clear();
            // First point of new line.
            line.insert(fst.1);
            line.insert(fst.2);
        }
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_points_on_line() {
        assert_eq!(0, max_points_on_line(&[]));
        assert_eq!(1, max_points_on_line(&[(1, 1)]));
        assert_eq!(2, max_points_on_line(&[(1, 1), (2, 2)]));
        assert_eq!(
            4,
            max_points_on_line(&[(1, 1), (3, 2), (5, 3), (4, 1), (2, 3), (1, 4)])
        );
        // 2 parallel lines!
        assert_eq!(
            4,
            max_points_on_line(&[(1, 1), (3, 2), (4, 1), (2, 3), (1, 4), (2, 0)])
        );
    }

    #[test]
    fn test_check_straight_line() {
        assert!(check_straight_line(vec![vec![1,2],vec![2,3],vec![3,4],vec![4,5],vec![5,6],vec![6,7]]));
        assert!(!check_straight_line(vec![vec![1,1],vec![2,2],vec![3,4],vec![4,5],vec![5,6],vec![7,7]]));
        assert!(check_straight_line(vec![vec![0,0],vec![0,1],vec![0,-1]]));

        assert!(!check_straight_line(vec![vec![0,0],vec![0,5],vec![5,5],vec![5,0]]));
    }
}
