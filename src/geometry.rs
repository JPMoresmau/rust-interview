use itertools::Itertools;

type IntPoint = (i64, i64);

/// Calculate the slope between two points.
/// We return a float to not lose precision, which is NOT `Eq` or `Hash`.
fn slope(p1: &IntPoint, p2: &IntPoint) -> f64 {
    (p2.1 - p1.1) as f64 / (p2.0 - p1.0) as f64
}

/// Are the given points all on the same slope?
fn same_slope(points: &[&IntPoint]) -> bool {
    if points.len() < 3 {
        return true;
    }
    // Calculate the slopes for all combinations of 2 points.
    let mut slopes = points
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| (slope(p1, p2)));
    let fst = slopes.next().unwrap();
    for n in slopes {
        // Warning this may not work well in the face of rounding errors.
        if fst != n {
            return false;
        }
    }
    true
}

/// Number of points on the same line.
/// <https://buttondown.email/cassidoo/archive/9753/>
/// Cheating maybe because we're using itertools.
/// Not performance optimized.
pub fn max_points_on_line(points: &[IntPoint]) -> usize {
    // No point provided.
    if points.is_empty() {
        return 0;
    }
    // One point provided.
    if points.len() == 1 {
        return 1;
    }
    // Brute force approach: find ensembles with the same slope.
    let mut sz = points.len();
    while sz > 2 {
        for c in points.iter().combinations(sz) {
            if same_slope(&c) {
                return sz;
            }
        }
        // try smaller ensembles.
        sz -= 1;
    }
    2
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
}
