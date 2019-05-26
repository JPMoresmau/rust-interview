use std::collections::HashSet;


pub fn missing_number(nbs: &[i32], low :i32, high: i32) -> Option<i32>{
    missing_numbers(nbs,low,high).iter().cloned().next()
}

pub fn missing_numbers(nbs: &[i32], low :i32, high: i32) -> HashSet<i32>{
    let mut s: HashSet<i32> = (low .. high+1).collect();
    nbs.iter().for_each(|i| {
        if *i < low || *i > high {
            panic!("{} wasn't in range {}-{}", i, low, high);
        }
        s.remove(i);
    });
    s
}

pub fn duplicate(nbs: &[i32]) -> Option<i32>{
    duplicates(nbs).iter().cloned().next()
}

pub fn duplicates(nbs: &[i32]) -> HashSet<i32>{
    let mut s: HashSet<i32> = HashSet::new();
    let mut dup: HashSet<i32> = HashSet::new();
    nbs.iter().for_each(|i| {
        let changed = s.insert(*i);
        if !changed {
            dup.insert(*i);
        }
    });
    dup
}

pub fn range(nbs: &[i32]) -> (i32,i32) {
    if nbs.len() == 0 {
        panic!("Empty array");
    }
    let mut min = nbs[0];
    let mut max = nbs[0];
    nbs.iter().for_each(|i| {
        let v=*i;
        if v<min {
            min = v;
        }
        if v>max {
            max = v;
        }
    });
    (min,max)
}

pub fn pairs_product(nbs: &[i32], prod: i32) -> Vec<(i32,i32)> {
    let mut v = vec!();
    for i in 0..nbs.len() {
        for j in i+1..nbs.len(){
            let vi = nbs[i];
            let vj = nbs[j];
            if vi * vj == prod {
                v.push((vi,vj));
            }
        }
    }
    /*nbs.iter().for_each(|i| {
        nbs.iter().for_each(|j| {
            if j > i {
                let p = *i * *j;
                if p == prod {
                    v.push((*i,*j));
                }
            }
        });
    });*/
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_missing_number() {
        assert_eq!(None,missing_number(&vec!(1,2,3), 1, 3));
        assert_eq!(None,missing_number(&vec!(3,2,1), 1, 3));
        assert_eq!(Some(2),missing_number(&vec!(1,3), 1, 3));
        assert_eq!(Some(2),missing_number(&vec!(1,3,1,3), 1, 3));
        assert_eq!(Some(1),missing_number(&vec!(2,3), 1, 3));
        assert_eq!(Some(3),missing_number(&vec!(1,2), 1, 3));
        assert_eq!(Some(2),missing_number(&vec!(3,1), 1, 3));
        assert_eq!(Some(1),missing_number(&vec!(3,2), 1, 3));
        assert_eq!(Some(3),missing_number(&vec!(2,1), 1, 3));
        assert_eq!(Some(100),missing_number(&((1..100).collect::<Vec<i32>>()), 1, 100));
    }

    #[test]
    #[should_panic(expected ="4 wasn't in range 1-3" )]
    fn test_missing_number_panic() {
        assert_eq!(None,missing_number(&vec!(1,2,3,4), 1, 3));
    }

    #[test]
    fn test_duplicate(){
        assert_eq!(None,duplicate(&vec!(1,2,3)));
        assert_eq!(Some(1),duplicate(&vec!(1,2,3,1)));
        assert_eq!(Some(1),duplicate(&vec!(1,1,2,3)));
        assert_eq!(Some(3),duplicate(&vec!(1,2,3,3)));
    }

     #[test]
    fn test_duplicates(){
        assert_eq!(HashSet::new(),duplicates(&vec!(1,2,3)));
        assert_eq!(HashSet::from_iter(vec!(1).iter().cloned()),duplicates(&vec!(1,2,3,1)));
        assert_eq!(HashSet::from_iter(vec!(1).iter().cloned()),duplicates(&vec!(1,1,2,3)));
        assert_eq!(HashSet::from_iter(vec!(3).iter().cloned()),duplicates(&vec!(1,2,3,3)));
        assert_eq!(HashSet::from_iter(vec!(1,3).iter().cloned()),duplicates(&vec!(1,1,2,3,3)));
    }

    #[test]
    fn test_range(){
        assert_eq!((0,0),range(&vec!(0)));
        assert_eq!((0,1),range(&vec!(0,1)));
        assert_eq!((-1,1),range(&vec!(0,1,-1)));
    }

    #[test]
    #[should_panic(expected = "Empty array")]
    fn test_range_panic(){
        assert_eq!((0,0),range(&vec!()));
    }

    #[test]
    fn test_pairs_product(){
        let e : Vec<(i32,i32)> = vec!();
        assert_eq!(e,pairs_product(&vec!(),2));
        assert_eq!(vec!((1,2)),pairs_product(&vec!(1,2),2));
        assert_eq!(vec!((2,3)),pairs_product(&vec!(1,2,3),6));
        assert_eq!(vec!((2,6),(3,4)),pairs_product(&vec!(1,2,3,4,6),12));
    }
}