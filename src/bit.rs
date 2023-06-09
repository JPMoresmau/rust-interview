pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
    let mut i = 1;
    let max = a.max(b).max(c);
    //*[a, b, c].iter().max().unwrap();
    let mut flips = 0;
    loop {
        if (c & i) == i {
            if ((a | b) & i) != i {
                flips += 1;
            }
        } else {
            if (a & i) == i {
                flips += 1;
            }
            if (b & i) == i {
                flips += 1;
            }
        }
        if i > max || i == i32::MAX {
            break;
        }
        i <<= 1;
    }
    flips
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_flips() {
        assert_eq!(3, min_flips(2, 6, 5));
        assert_eq!(1, min_flips(4, 2, 7));
        assert_eq!(0, min_flips(1, 2, 3));
    }
}
