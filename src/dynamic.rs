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

pub enum Term {
    Plus,
    Times,
}

fn calc(t: &Term, a: &i32, b: &i32) -> i32 {
    match t {
        Term::Plus => a + b,
        Term::Times => a * b,
    }
}

/// Place parens to maximize operations.
/// <https://www.youtube.com/watch?v=TDo3r5M1LNo>
pub fn parens(nbs: &[i32], ts: &[Term]) -> i32 {
    let mut dp = vec![vec![(0, 0); nbs.len()]; nbs.len()];
    for (ix, v) in nbs.iter().enumerate() {
        dp[ix][ix] = (*v, *v);
    }
    for j in 1..nbs.len() {
        for i in 0..(nbs.len() - j) {
            let mut max = i32::MIN;
            let mut min = i32::MAX;
            for k in i..i + j {
                let (min0, max0) = dp[i][k];
                let (min1, max1) = dp[k + 1][i + j];
                for m0 in &[min0, max0] {
                    for m1 in &[min1, max1] {
                        let r = calc(&ts[k], m0, m1);
                        max = max.max(r);
                        min = min.min(r);
                    }
                }
            }
            dp[i][i + j] = (min, max);
        }
    }
    dp[0][nbs.len() - 1].1
}

/// <https://www.youtube.com/watch?v=i9OAOk0CUQE>
pub fn rod(values: &[u32]) -> u32 {
    let mut dp = vec![0];
    for l in 0..values.len() {
        let m = (0..=l).map(|p| values[p] + dp[l - p]).max().unwrap_or(0);
        dp.push(m);
    }
    dp[values.len()]
}

/// <https://www.youtube.com/watch?v=i9OAOk0CUQE>
pub fn subset_sum(sum: usize, values: &[usize]) -> bool {
    let mut dp = vec![vec![false; sum + 1]; values.len() + 1];
    for v in dp.iter_mut() {
        v[0] = true;
    }
    for (i, v) in values.iter().enumerate() {
        for t in 0..=sum {
            dp[i + 1][t] = dp[i][t] || (*v <= t && dp[i][t - v]);
        }
    }
    dp[values.len()][sum]
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

    #[test]
    fn test_parens() {
        assert_eq!(
            88,
            parens(&[7, 4, 3, 5], &[Term::Plus, Term::Times, Term::Plus])
        );
        assert_eq!(
            15,
            parens(&[7, -4, 3, -5], &[Term::Plus, Term::Times, Term::Plus])
        );
    }

    #[test]
    fn test_rod() {
        assert_eq!(33, rod(&[1, 10, 13, 18, 20, 31, 32]))
    }

    #[test]
    fn test_subset_sum() {
        assert!(subset_sum(21, &[2, 5, 7, 8, 9]));
        assert!(!subset_sum(25, &[2, 5, 7, 8, 9]))
    }
}
