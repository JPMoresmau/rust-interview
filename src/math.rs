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

///! calculate pi with algebraic/geometric mean
pub fn pi(n: usize) -> f64 {
    let mut a : f64 = 1.0;
    let two : f64= 2.0;
    let mut g = 1.0 / two.sqrt();
    let mut s = 0.0;
    let mut k = 1;
    while k<=n  {
       
        let a1 = (a+g)/two;
        let g1 = (a*g).sqrt();
        a = a1;
        g = g1;
        s += (a.powi(2)-g.powi(2)) * two.powi((k+1) as i32);
        k += 1;
         

    }

    4.0 * a.powi(2) / (1.0-s)
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
        approx_eq(-80.7109, angle_difference(-78.3251 , -159.036));
        approx_eq(-139.5832, angle_difference(-70099.74233810938, 29840.67437876723));
        approx_eq(-72.3439, angle_difference(-165313.6666297357, 33693.9894517456));
        approx_eq(-161.5029, angle_difference(1174.8380510598456, -154146.66490124757));
        approx_eq(37.2988, angle_difference(60175.77306795546, 42213.07192354373));
    }

    // approximate equality on floats.
    // see also https://crates.io/crates/float-cmp
    fn approx_eq(f1: f64, f2: f64) {
        assert!((f2-f1).abs() < 0.0001, "{} != {}", f1, f2)
    }
}