

#[derive(Eq,PartialEq,Debug)]
pub struct MySet {
    data: Vec<i32>
}

impl MySet {

    pub fn new() -> MySet {
         MySet {data: Vec::new()}
    }

    pub fn from_slice(nbs: &[i32]) -> MySet {
        let mut v = vec!();
        for i in nbs {
            v.push(*i);
        }
        MySet {data: v}
    }

    pub fn from_vec(v: Vec<i32>) -> MySet {
        MySet {data: v.clone()}
    }

    pub fn contains(&self, i:i32) -> bool {
        self.data.contains(&i)
    }

    pub fn insert(&mut self, i:i32) -> bool {
        if self.contains(i){
            return false;
        }
        self.data.push(i);
        true
    }

    
    pub fn remove(&mut self, i: &i32) -> bool {
        let idx=self.data.iter().enumerate().find(|(_,n)| **n == *i);
        match idx {
            None => false,
            Some((ix,_)) => {
                self.data.remove(ix);
                true
            },
        }
    }

    pub fn first(&self) ->  Option<i32> {
        self.data.iter().next().map(|i| *i)
    }
}


pub fn missing_number(nbs: &[i32], low :i32, high: i32) -> Option<i32>{
    missing_numbers(nbs,low,high).first()
}

pub fn missing_numbers(nbs: &[i32], low :i32, high: i32) -> MySet{
    let mut s = MySet::from_vec((low .. high+1).collect());
    nbs.iter().for_each(|i| {
        if *i < low || *i > high {
            panic!("{} wasn't in range {}-{}", i, low, high);
        }
        s.remove(i);
    });
    s
}

pub fn duplicate(nbs: &[i32]) -> Option<i32>{
    duplicates(nbs).first()
}

pub fn duplicates(nbs: &[i32]) -> MySet{
    let mut s = MySet::new();
    let mut dup = MySet::new();
    nbs.iter().for_each(|i| {
        let changed = s.insert(*i);
        if !changed {
            dup.insert(*i);
        }
    });
    dup
}

pub fn dedup(nbs: &mut Vec<i32>) {
    let mut s = MySet::new();
    nbs.retain(|i| s.insert(*i))
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

pub fn quick_sort(nbs: &mut Vec<i32>) {
    if nbs.len()>1 {
        quick_sort_partial(nbs, 0, nbs.len()-1);
    }
}

pub fn quick_sort_partial(nbs: &mut Vec<i32>,low: usize, high: usize) {
    if low < high {
        let p = partition(nbs, low, high);
        quick_sort_partial(nbs, low, p);
        quick_sort_partial(nbs, p+1, high);
    }
}

fn partition(nbs: &mut Vec<i32>,low: usize, high: usize) -> usize {
    /* Lomuto
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
    /* Hoare */
    let pivot : i32 = nbs[low + (high-low)/2];
    let mut i = low ;
    let mut j = high;
    loop {
        while nbs[i] < pivot {i=i+1};
        while nbs[j] > pivot {j=j-1};
        if i >= j {
            break j;
        }
        swap(nbs,i,j);
        i=i+1;
        j=j-1;
    }
}

fn swap(nbs: &mut Vec<i32>,low: usize, high: usize) {
    let tmp : i32 = nbs[low];
    nbs[low] = nbs[high];
    nbs[high] = tmp;
}

pub fn reverse(nbs: &mut Vec<i32>) {
    if nbs.len()>0 {
        let mut i = 0;
        let mut j = nbs.len()-1;
        while i<j {
            swap(nbs,i,j);
            i=i+1;
            j=j-1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(MySet::new(),duplicates(&vec!(1,2,3)));
        assert_eq!(MySet::from_vec(vec!(1)),duplicates(&vec!(1,2,3,1)));
        assert_eq!(MySet::from_vec(vec!(1)),duplicates(&vec!(1,1,2,3)));
        assert_eq!(MySet::from_vec(vec!(3)),duplicates(&vec!(1,2,3,3)));
        assert_eq!(MySet::from_vec(vec!(1,3)),duplicates(&vec!(1,1,2,3,3)));
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

    #[test]
    fn test_quick_sort(){
        let mut nbs = vec!(3,2,35,1,-2,100);
        quick_sort(&mut nbs);
        assert_eq!(vec!(-2,1,2,3,35,100),nbs);
    }

    #[test]
    fn test_dedup(){
        let mut nbs = vec!(3,1,1,2,3);
        dedup(&mut nbs);
        assert_eq!(vec!(3,1,2),nbs);
    }

    #[test]
    fn test_reverse(){
        let mut nbs = vec!(1,2,3,4);
        reverse(&mut nbs);
        assert_eq!(vec!(4,3,2,1),nbs);
        let mut nbs = vec!(1,2,3,4,5);
        reverse(&mut nbs);
        assert_eq!(vec!(5,4,3,2,1),nbs);
        let mut nbs = vec!();
        reverse(&mut nbs);
        assert_eq!(vec!() as Vec<i32>,nbs);
        let mut nbs = vec!(1);
        reverse(&mut nbs);
        assert_eq!(vec!(1),nbs);
    }
}