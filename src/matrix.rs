pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let mut neg_index = 0;
    let mut negatives = 0;
    let len = grid[0].len();
    for i in (0..grid.len()).rev() {
        while neg_index < len && grid[i][neg_index] >= 0 {
            neg_index += 1;
        }
        let neg = len - neg_index;
        negatives += neg as i32;
        if neg == 0 {
            break;
        }
    }
    negatives
}

// https://leetcode.com/problems/equal-row-and-column-pairs/
pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
    let mut eq = 0;
    let n = grid.len();
    let mut transposed = Vec::with_capacity(n);
    for i in 0..n {
        let mut sz: u128 = 0;
        for row in &grid {
            sz *= 100_001;
            sz += row[i] as u128;
        }
        transposed.push(sz);
    }

    for row in grid.into_iter() {
        let mut sz: u128 = 0;
        for i in row {
            sz *= 100_001;
            sz += i as u128;
        }
        for col in &transposed {
            if &sz == col {
                eq += 1;
            }
        }
    }
    eq
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_negatives() {
        assert_eq!(
            8,
            count_negatives(vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3]
            ])
        );
        assert_eq!(0, count_negatives(vec![vec![3, 2], vec![1, 0]]));
    }

    #[test]
    fn test_equal_pairs() {
        assert_eq!(
            1,
            equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]])
        );
        assert_eq!(
            3,
            equal_pairs(vec![
                vec![3, 1, 2, 2],
                vec![1, 4, 4, 5],
                vec![2, 4, 2, 2],
                vec![2, 4, 2, 2]
            ])
        );
    }
}
