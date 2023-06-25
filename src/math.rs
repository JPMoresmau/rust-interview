//! Maths functions

use std::f64;

/// Calculate difference between two bearings, in -180 to 180 degrees range
/// Submitted to http://www.rosettacode.org/wiki/Angle_difference_between_two_bearings#Rust
pub fn angle_difference(bearing1: f64, bearing2: f64) -> f64 {
    let diff = (bearing2 - bearing1) % 360.0;
    if diff < -180.0 {
        360.0 + diff
    } else if diff > 180.0 {
        -360.0 + diff
    } else {
        diff
    }
}

/// Calculate pi with algebraic/geometric mean
pub fn pi(n: usize) -> f64 {
    let mut a: f64 = 1.0;
    let two: f64 = 2.0;
    let mut g = 1.0 / two.sqrt();
    let mut s = 0.0;
    let mut k = 1;
    while k <= n {
        let a1 = (a + g) / two;
        let g1 = (a * g).sqrt();
        a = a1;
        g = g1;
        s += (a.powi(2) - g.powi(2)) * two.powi((k + 1) as i32);
        k += 1;
    }

    4.0 * a.powi(2) / (1.0 - s)
}

/*
pub fn pi(n: usize) -> BigRational {
    let mut a = BigRational::from_i64(1).unwrap();
    let two = BigRational::from_i64(2).unwrap();
    let mut g = a / two.sqrt();
    let mut s = 0.0;
    let mut k : usize = 1;
//&& (a-g).abs() > f64::EPSILON
    while k<=n  {

        let a1 = (a+g)/two;
        let g1 = (a*g).sqrt();
        a = a1;
        g = g1;
        s += (pow(a,2)-pow(g,2)) * pow(two,k+1);
        k += 1;


    }

    4.0 * a.powi(2) / (1.0-s)
}*/

/// https://leetcode.com/problems/maximum-value-at-a-given-index-in-a-bounded-array/
pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
    let floor = 1.max(max_sum / n - ((index - n).abs().max(index)));
    let mut sum = floor * n;
    let mut amp = 1;
    let mut res = floor;
    if sum < max_sum {
        'outer: loop {
            res += 1;
            sum += n.min(index + amp + 1) - 0.max(index - amp);
            if sum >= max_sum {
                break 'outer;
            }
            if index - amp > 0 || index + amp < n - 1 {
                amp += 1;
            }
        }
    }
    res
}

/// https://leetcode.com/problems/summary-ranges/
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    let mut ranges = Vec::new();
    let mut range = |start: i32, current: i32| {
        if current == start {
            ranges.push(current.to_string());
        } else {
            ranges.push(format!("{start}->{current}"));
        }
    };
    if !nums.is_empty() {
        let mut start = nums[0];
        let mut current = start;
        for i in &nums[1..] {
            if *i > current + 1 {
                range(start, current);
                start = *i;
            }
            current = *i;
        }
        range(start, current);
    }
    ranges
}

/// <https://leetcode.com/problems/find-the-highest-altitude/>
pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut alt = 0;
    let mut max = 0;
    for g in gain {
        alt += g;
        max = max.max(alt);
    }
    max
}

pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
    if prices.is_empty() {
        return 0;
    }
    let hold = -prices[0];

    prices
        .into_iter()
        .skip(1)
        .fold((hold, 0), |(hold, free), p| {
            let tmp = hold;
            (hold.max(free - p), free.max(tmp + p - fee))
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_angle_difference() {
        assert_eq!(25.00, angle_difference(20.00, 45.00));
        assert_eq!(90.00, angle_difference(-45.00, 45.00));
        assert_eq!(175.00, angle_difference(-85.00, 90.00));
        assert_eq!(-175.00, angle_difference(-95.00, 90.00));
        assert_eq!(170.00, angle_difference(-45.00, 125.00));
        assert_eq!(-170.00, angle_difference(-45.00, 145.00));
        approx_eq(-118.1184, angle_difference(29.4803, -88.6381));
        approx_eq(-80.7109, angle_difference(-78.3251, -159.036));
        approx_eq(
            -139.5832,
            angle_difference(-70099.74233810938, 29840.67437876723),
        );
        approx_eq(
            -72.3439,
            angle_difference(-165313.6666297357, 33693.9894517456),
        );
        approx_eq(
            -161.5029,
            angle_difference(1174.8380510598456, -154146.66490124757),
        );
        approx_eq(
            37.2988,
            angle_difference(60175.77306795546, 42213.07192354373),
        );
    }

    // approximate equality on floats.
    // see also https://crates.io/crates/float-cmp
    fn approx_eq(f1: f64, f2: f64) {
        assert!((f2 - f1).abs() < 0.0001, "{} != {}", f1, f2)
    }

    #[test]
    fn test_max_value() {
        assert_eq!(2, max_value(4, 2, 6));
        assert_eq!(3, max_value(6, 1, 10));
        assert_eq!(7, max_value(3, 2, 18));
        assert_eq!(1, max_value(4, 0, 4));
        assert_eq!(10102750, max_value(9, 0, 90924720));
        assert_eq!(271698267, max_value(3, 0, 815094800));
        assert_eq!(8, max_value(5, 4, 30));
        assert_eq!(11049, max_value(8067, 368, 59432211));
    }

    #[test]
    fn test_summary_ranges() {
        assert_eq!(
            vec!["0->2", "4->5", "7"],
            summary_ranges(vec![0, 1, 2, 4, 5, 7])
        );
        assert_eq!(
            vec!["0", "2->4", "6", "8->9"],
            summary_ranges(vec![0, 2, 3, 4, 6, 8, 9])
        );
    }
    #[test]
    fn test_largest_altitude() {
        assert_eq!(1, largest_altitude(vec![-5, 1, 5, 0, -7]));
        assert_eq!(0, largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]));
    }

    #[test]
    fn test_max_profit() {
        assert_eq!(8, max_profit(vec![1, 3, 2, 8, 4, 9], 2));
        assert_eq!(6, max_profit(vec![1, 3, 7, 5, 10, 3], 3));
    }
}
