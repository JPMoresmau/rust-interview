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

pub struct Solution {}

impl Solution {
    /// <https://leetcode.com/problems/number-of-increasing-paths-in-a-grid/>
    const M: i64 = 1_000_000_007;

    pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dps = vec![vec![0_i64; n]; m];

        for i in 0..m {
            for j in 0..n {
                Solution::dps(&grid, i, j, &mut dps);
            }
        }
        dps.into_iter().fold(0, |cnt, v| {
            v.into_iter().fold(cnt, |cnt2, v| (cnt2 + v) % Solution::M)
        }) as i32
    }

    fn dps(grid: &Vec<Vec<i32>>, i: usize, j: usize, dps: &mut Vec<Vec<i64>>) -> i64 {
        if dps[i][j] == 0 {
            let v = grid[i][j];
            let mut cnt = 1;
            if i > 0 {
                cnt += Solution::dps1(grid, i - 1, j, dps, v);
            }
            if j > 0 {
                cnt += Solution::dps1(grid, i, j - 1, dps, v);
            } 
            if i < grid.len() - 1 {
                cnt += Solution::dps1(grid, i + 1, j, dps, v);
            } 
            if j < grid[0].len() - 1 {
                cnt += Solution::dps1(grid, i, j + 1, dps, v);
            }
            
            dps[i][j] = cnt % Solution::M;
        }
        dps[i][j]
    }

    fn dps1(grid: &Vec<Vec<i32>>, i: usize, j: usize, dps: &mut Vec<Vec<i64>>, v: i32) -> i64 {
        if grid[i][j] < v {
            Solution::dps(grid, i, j, dps)
        } else {
            0
        }
    }
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

    #[test]
    fn test_count_paths() {
        assert_eq!(8, Solution::count_paths(vec![vec![1, 1], vec![3, 4]]));
        assert_eq!(3, Solution::count_paths(vec![vec![1], vec![2]]));
    }
}
