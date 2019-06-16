//! Integer arrays problems

/// Our own trivial implementation of an integer set to avoid using HashSet
#[derive(Eq, PartialEq, Debug)]
pub struct IntSet {
    data: Vec<i32>,
}

/// implementation of set
impl IntSet {

    /// new empty set
    pub fn new() -> IntSet {
        IntSet { data: Vec::new() }
    }

    /// create a set from a slice
    pub fn from_slice(nbs: &[i32]) -> IntSet {
        let mut v = vec![];
        for i in nbs {
            v.push(*i);
        }
        IntSet { data: v }
    }

    /// create a set from a vec
    pub fn from_vec(v: Vec<i32>) -> IntSet {
        IntSet { data: v.clone() }
    }

    /// does the set contain the given integer?
    pub fn contains(&self, i: i32) -> bool {
        self.data.contains(&i)
    }

    /// insert a new integer
    /// return true if the set was changed (didn't contain the integer)
    pub fn insert(&mut self, i: i32) -> bool {
        if self.contains(i) {
            return false;
        }
        self.data.push(i);
        true
    }

    /// remove an integer
    /// return true if the set was changed (contained the integer)
    pub fn remove(&mut self, i: &i32) -> bool {
        let idx = self.data.iter().enumerate().find(|(_, n)| **n == *i);
        match idx {
            None => false,
            Some((ix, _)) => {
                self.data.remove(ix);
                true
            }
        }
    }

    /// get the first integer in the set, or None if the set is empty
    pub fn first(&self) -> Option<i32> {
        self.data.iter().next().map(|i| *i)
    }
}

/// find the first missing number in the array in the given range
pub fn missing_number(nbs: &[i32], low: i32, high: i32) -> Option<i32> {
    missing_numbers(nbs, low, high).first()
}

/// find all missing numbers in the array in the given range
pub fn missing_numbers(nbs: &[i32], low: i32, high: i32) -> IntSet {
    let mut s = IntSet::from_vec((low..high + 1).collect());
    nbs.iter().for_each(|i| {
        if *i < low || *i > high {
            panic!("{} wasn't in range {}-{}", i, low, high);
        }
        s.remove(i);
    });
    s
}

/// find first duplicate in the given array
pub fn duplicate(nbs: &[i32]) -> Option<i32> {
    duplicates(nbs).first()
}

/// find all duplicates in the given array
pub fn duplicates(nbs: &[i32]) -> IntSet {
    let mut s = IntSet::new();
    let mut dup = IntSet::new();
    nbs.iter().for_each(|i| {
        let changed = s.insert(*i);
        if !changed {
            dup.insert(*i);
        }
    });
    dup
}

/// remove duplicates
pub fn dedup(nbs: &mut Vec<i32>) {
    let mut s = IntSet::new();
    nbs.retain(|i| s.insert(*i))
}

/// return min and max elements
pub fn range(nbs: &[i32]) -> (i32, i32) {
    if nbs.len() == 0 {
        panic!("Empty array");
    }
    let mut min = nbs[0];
    let mut max = nbs[0];
    nbs.iter().for_each(|i| {
        let v = *i;
        if v < min {
            min = v;
        }
        if v > max {
            max = v;
        }
    });
    (min, max)
}

/// find all pairs whose product match the given number
pub fn pairs_product(nbs: &[i32], prod: i32) -> Vec<(i32, i32)> {
    let mut v = vec![];
    for i in 0..nbs.len() {
        for j in i + 1..nbs.len() {
            let vi = nbs[i];
            let vj = nbs[j];
            if vi * vj == prod {
                v.push((vi, vj));
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

/// quick sort in place
pub fn quick_sort(nbs: &mut Vec<i32>) {
    if nbs.len() > 1 {
        quick_sort_partial(nbs, 0, nbs.len() - 1);
    }
}

/// quick sort step
pub fn quick_sort_partial(nbs: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let p = partition(nbs, low, high);
        quick_sort_partial(nbs, low, p);
        quick_sort_partial(nbs, p + 1, high);
    }
}

/// quick sort partition step
fn partition(nbs: &mut Vec<i32>, low: usize, high: usize) -> usize {
    // Lomuto
    /*
    let pivot : i32 = nbs[high];
    let mut i = low;
    for j in low..high {
        if nbs[j]<pivot {
            swap(nbs,i,j);
            i = i+1;
        }
    }
    swap(nbs,i,high);
    i*/
    // Hoare
    let pivot: i32 = nbs[low + (high - low) / 2];
    let mut i = low;
    let mut j = high;
    loop {
        while nbs[i] < pivot {
            i = i + 1
        }
        while nbs[j] > pivot {
            j = j - 1
        }
        if i >= j {
            break j;
        }
        swap(nbs, i, j);
        i = i + 1;
        j = j - 1;
    }
}

/// bubble sort
pub fn bubble_sort(nbs: &mut Vec<i32>) {
    loop {
        let mut changed = false;
        for i in 0..nbs.len()-1 {
            if nbs[i]>nbs[i+1]{
                swap(nbs,i,i+1);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
}

/// insertion sort
pub fn insertion_sort(nbs: &mut Vec<i32>) {
    for i in 1..nbs.len() {
        let mut j = i;
        while j > 0 && nbs[j-1]>nbs[j]{
            swap(nbs,j-1,j);
            j-=1;
        }
    }
}

/// merge sort
pub fn merge_sort(nbs: &mut Vec<i32>) {
    let v = merge_sort_step(nbs,0,nbs.len());
    for i in 0..nbs.len(){
        nbs[i] = v[i];
    }
}

/// merge sort step, returning a new sorted vector
fn merge_sort_step(nbs: &Vec<i32>,low: usize, high: usize) -> Vec<i32>{
    if low<high-1 {
        let middle = low + (high-low) / 2;
        let v1 = merge_sort_step(nbs,low,middle);
        let v2 = merge_sort_step(nbs,middle,high);
        let mut i1 = 0;
        let mut i2 = 0;
        let mut ret = Vec::with_capacity(high-low);
        while i1<v1.len() || i2<v2.len() {
            if i1<v1.len() && (i2==v2.len() || v1[i1]<v2[i2]){
                ret.push(v1[i1]);
                i1+=1;
            } else {
                ret.push(v2[i2]);
                i2+=1;
            }
        };
        ret
    } else {
        vec!(nbs[low])
    }
}

/// swap two numbers
fn swap(nbs: &mut Vec<i32>, low: usize, high: usize) {
    let tmp: i32 = nbs[low];
    nbs[low] = nbs[high];
    nbs[high] = tmp;
}

/// reverse in place
pub fn reverse(nbs: &mut Vec<i32>) {
    if nbs.len() > 0 {
        let mut i = 0;
        let mut j = nbs.len() - 1;
        while i < j {
            swap(nbs, i, j);
            i = i + 1;
            j = j - 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_number() {
        assert_eq!(None, missing_number(&vec!(1, 2, 3), 1, 3));
        assert_eq!(None, missing_number(&vec!(3, 2, 1), 1, 3));
        assert_eq!(Some(2), missing_number(&vec!(1, 3), 1, 3));
        assert_eq!(Some(2), missing_number(&vec!(1, 3, 1, 3), 1, 3));
        assert_eq!(Some(1), missing_number(&vec!(2, 3), 1, 3));
        assert_eq!(Some(3), missing_number(&vec!(1, 2), 1, 3));
        assert_eq!(Some(2), missing_number(&vec!(3, 1), 1, 3));
        assert_eq!(Some(1), missing_number(&vec!(3, 2), 1, 3));
        assert_eq!(Some(3), missing_number(&vec!(2, 1), 1, 3));
        assert_eq!(
            Some(100),
            missing_number(&((1..100).collect::<Vec<i32>>()), 1, 100)
        );
    }

    #[test]
    #[should_panic(expected = "4 wasn't in range 1-3")]
    fn test_missing_number_panic() {
        assert_eq!(None, missing_number(&vec!(1, 2, 3, 4), 1, 3));
    }

    #[test]
    fn test_duplicate() {
        assert_eq!(None, duplicate(&vec!(1, 2, 3)));
        assert_eq!(Some(1), duplicate(&vec!(1, 2, 3, 1)));
        assert_eq!(Some(1), duplicate(&vec!(1, 1, 2, 3)));
        assert_eq!(Some(3), duplicate(&vec!(1, 2, 3, 3)));
    }

    #[test]
    fn test_duplicates() {
        assert_eq!(IntSet::new(), duplicates(&vec!(1, 2, 3)));
        assert_eq!(IntSet::from_vec(vec!(1)), duplicates(&vec!(1, 2, 3, 1)));
        assert_eq!(IntSet::from_vec(vec!(1)), duplicates(&vec!(1, 1, 2, 3)));
        assert_eq!(IntSet::from_vec(vec!(3)), duplicates(&vec!(1, 2, 3, 3)));
        assert_eq!(
            IntSet::from_vec(vec!(1, 3)),
            duplicates(&vec!(1, 1, 2, 3, 3))
        );
    }

    #[test]
    fn test_range() {
        assert_eq!((0, 0), range(&vec!(0)));
        assert_eq!((0, 1), range(&vec!(0, 1)));
        assert_eq!((-1, 1), range(&vec!(0, 1, -1)));
    }

    #[test]
    #[should_panic(expected = "Empty array")]
    fn test_range_panic() {
        assert_eq!((0, 0), range(&vec!()));
    }

    #[test]
    fn test_pairs_product() {
        let e: Vec<(i32, i32)> = vec![];
        assert_eq!(e, pairs_product(&vec!(), 2));
        assert_eq!(vec!((1, 2)), pairs_product(&vec!(1, 2), 2));
        assert_eq!(vec!((2, 3)), pairs_product(&vec!(1, 2, 3), 6));
        assert_eq!(
            vec!((2, 6), (3, 4)),
            pairs_product(&vec!(1, 2, 3, 4, 6), 12)
        );
    }

    #[test]
    fn test_quick_sort() {
        test_sort(&quick_sort);
    }

    #[test]
    fn test_bubble_sort() {
        test_sort(&bubble_sort);
    }

    #[test]
    fn test_insertion_sort() {
        test_sort(&insertion_sort);
    }

    #[test]
    fn test_merge_sort() {
        test_sort(&merge_sort);
    }

    fn test_sort(f: &Fn(&mut Vec<i32>)) {
        let mut nbs = vec![3, 2, 1];
        f(&mut nbs);
        assert_eq!(vec!(1, 2, 3), nbs);
        let mut nbs = vec![1, 2, 3];
        f(&mut nbs);
        assert_eq!(vec!(1, 2, 3), nbs);
        let mut nbs = vec![3, 2, 35, 1, -2, 100];
        f(&mut nbs);
        assert_eq!(vec!(-2, 1, 2, 3, 35, 100), nbs);
    }

    #[test]
    fn test_dedup() {
        let mut nbs = vec![3, 1, 1, 2, 3];
        dedup(&mut nbs);
        assert_eq!(vec!(3, 1, 2), nbs);
    }

    #[test]
    fn test_reverse() {
        let mut nbs = vec![1, 2, 3, 4];
        reverse(&mut nbs);
        assert_eq!(vec!(4, 3, 2, 1), nbs);
        let mut nbs = vec![1, 2, 3, 4, 5];
        reverse(&mut nbs);
        assert_eq!(vec!(5, 4, 3, 2, 1), nbs);
        let mut nbs = vec![];
        reverse(&mut nbs);
        assert_eq!(vec!() as Vec<i32>, nbs);
        let mut nbs = vec![1];
        reverse(&mut nbs);
        assert_eq!(vec!(1), nbs);
    }
}