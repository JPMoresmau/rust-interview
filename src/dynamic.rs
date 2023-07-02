/// <https://www.youtube.com/watch?v=r4-cftqTcdI>
pub fn bowling(pins: &[i32]) -> i32 {
    let mut max = 0;
    let mut previous0 = 0;
    let mut previous1 = 0;
    let mut previous_weight = 0;
    for w in pins {
        max = max.max(previous0 + previous_weight * w).max(previous1 + w);
        previous0 = previous1;
        previous1 = max;
        previous_weight = *w;
    }

    max
}

/// <https://www.youtube.com/watch?v=KLBCUx1is2c>
pub fn lcs<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: Eq + Copy,
{
    let mut lcs = vec![];
    if a.is_empty() | b.is_empty() {
        return lcs;
    }
    let mut dp = vec![vec![0; a.len() + 1]; b.len() + 1];
    for i in (0..b.len()).rev() {
        for j in (0..a.len()).rev() {
            if b[i] == a[j] {
                dp[i][j] = dp[i + 1][j + 1] + 1;
            } else {
                dp[i][j] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }
    let mut i = 0;
    let mut j = 0;
    while i < b.len() && j < a.len() {
        if b[i] == a[j] {
            lcs.push(b[i]);
            i += 1;
            j += 1;
        } else if dp[i + 1][j] > dp[i][j + 1] {
            i += 1;
        } else {
            j += 1;
        }
    }
    lcs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bowling() {
        assert_eq!(0, bowling(&[]));
        assert_eq!(4, bowling(&[4]));
        assert_eq!(0, bowling(&[-4]));
        assert_eq!(6, bowling(&[2, 3]));
        assert_eq!(110, bowling(&[1, 1, 9, 9, 2, -5, -5]));
        assert_eq!(110, bowling(&[-3, 1, 1, 9, 9, 2, -5, -5]));
    }

    #[test]
    fn test_lcs() {
        assert_eq!(vec!['E', 'L'], lcs(&['T', 'E', 'L'], &['H', 'E', 'L', 'L']));
        assert_eq!(Vec::<char>::new(), lcs(&[], &['H', 'E', 'L', 'L']));
        assert_eq!(Vec::<char>::new(), lcs(&['T', 'E', 'L'], &[]));
        assert_eq!(
            vec!['H', 'I'],
            lcs(&['T', 'H', 'E', 'I', 'R'], &['H', 'A', 'B', 'I', 'T'])
        );
        assert_eq!(
            vec!['I', 'E', 'L', 'L', 'O'],
            lcs(
                &['H', 'I', 'E', 'R', 'O', 'G', 'L', 'Y', 'P', 'H', 'O', 'L', 'O', 'G', 'Y'],
                &['M', 'I', 'C', 'H', 'A', 'E', 'L', 'A', 'N', 'G', 'E', 'L', 'O']
            )
        );
    }
}
