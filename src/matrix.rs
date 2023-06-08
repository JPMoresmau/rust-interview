pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
    let mut neg_index = 0;
    let mut negatives = 0;
    let len = grid[0].len();
    for i in (0..grid.len()).rev(){
        while neg_index < len && grid[i][neg_index]>=0{
            neg_index+=1;
        }
        let neg = len - neg_index;
        negatives += neg as i32;
        if neg == 0 {
            break;
        }
    }
    negatives
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
}
