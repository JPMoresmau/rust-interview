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
}
