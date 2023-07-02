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

/// Longest common subsequence.
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

/// Longest incrementing subsequence.
/// <https://www.youtube.com/watch?v=KLBCUx1is2c>
pub fn lis<T>(a: &[T]) -> Vec<T>
where
    T: Ord + Copy,
{
    if a.is_empty() {
        return vec![];
    }
    let mut dp = vec![vec![]; a.len()];
    let mut lis = vec![];
    for i in (0..a.len()).rev() {
        dp[i].push(a[i]);
        for j in (i + 1)..a.len() {
            if a[j] > a[i] && dp[j].len() >= dp[i].len() {
                let v;
                // SAFETY: i != j && dp[i] is distinct from dp[j] so modifying dp[i] while reading dp[j] is safe.
                unsafe {
                    v = &mut *(&mut dp[i] as *mut Vec<T>);
                }
                v.clear();
                v.push(a[i]);
                v.extend(&dp[j]);
            }
        }
        if dp[i].len() > lis.len() {
            lis.clear();
            lis.extend(&dp[i]);
        }
    }

    lis
}

/// Coins game.
/// <https://www.youtube.com/watch?v=KLBCUx1is2c>
#[derive(Debug, Default, Clone)]
struct Pick {
    me: u32,
    you: u32,
}

pub fn coins(coins: &[u32]) -> u32 {
    let mut dp = vec![vec![Pick::default(); coins.len()]; coins.len()];
    for (ix, i) in coins.iter().enumerate() {
        dp[ix][ix].me = *i;
    }
    for p in 0..coins.len() {
        for i in 0..coins.len() {
            let j = i + p + 1;
            if j < coins.len() {
                dp[i][j].me = (dp[i + 1][j].you + coins[i]).max(dp[i][j - 1].you + coins[j]);
                dp[i][j].you = dp[i + 1][j].me.min(dp[i][j - 1].me);
            }
        }
    }

    dp[0][dp.len() - 1].me
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

    #[test]
    fn test_lis() {
        assert_eq!(Vec::<char>::new(), lis(&[]));
        assert_eq!(vec!['A'], lis(&['H', 'A']));
        assert_eq!(
            vec!['E', 'M', 'P', 'T', 'Y'],
            lis(&['E', 'M', 'P', 'A', 'T', 'H', 'Y'])
        );
        assert_eq!(
            vec!['A', 'B', 'O', 'R', 'T'],
            lis(&['C', 'A', 'R', 'B', 'O', 'H', 'Y', 'D', 'R', 'A', 'T', 'E'])
        );
    }

    #[test]
    fn test_coins() {
        assert_eq!(105, coins(&[5, 10, 100, 25]));
    }
}
