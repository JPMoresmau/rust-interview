//! Integer arrays problems

/// Our own trivial implementation of an integer set to avoid using HashSet
#[derive(Eq, PartialEq, Debug, Default)]
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
        IntSet { data: v }
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
        self.data.first().copied()
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
    if nbs.is_empty() {
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
fn partition(nbs: &mut [i32], low: usize, high: usize) -> usize {
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
            i += 1;
        }
        while nbs[j] > pivot {
            j -= 1;
        }
        if i >= j {
            break j;
        }
        nbs.swap(i, j);
        i += 1;
        j -= 1;
    }
}

/// bubble sort
pub fn bubble_sort(nbs: &mut Vec<i32>) {
    loop {
        let mut changed = false;
        for i in 0..nbs.len() - 1 {
            if nbs[i] > nbs[i + 1] {
                nbs.swap(i, i + 1);
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
        while j > 0 && nbs[j - 1] > nbs[j] {
            nbs.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// merge sort
pub fn merge_sort(nbs: &mut Vec<i32>) {
    let l = nbs.len();
    let v = merge_sort_step(nbs, 0, l);
    nbs.copy_from_slice(&v[..l]);
}

/// merge sort step, returning a new sorted vector
fn merge_sort_step(nbs: &Vec<i32>, low: usize, high: usize) -> Vec<i32> {
    if low < high - 1 {
        let middle = low + (high - low) / 2;
        let v1 = merge_sort_step(nbs, low, middle);
        let v2 = merge_sort_step(nbs, middle, high);
        let mut i1 = 0;
        let mut i2 = 0;
        let mut ret = Vec::with_capacity(high - low);
        while i1 < v1.len() || i2 < v2.len() {
            if i1 < v1.len() && (i2 == v2.len() || v1[i1] < v2[i2]) {
                ret.push(v1[i1]);
                i1 += 1;
            } else {
                ret.push(v2[i2]);
                i2 += 1;
            }
        }
        ret
    } else {
        vec![nbs[low]]
    }
}

/// heap sort
pub fn heap_sort(nbs: &mut Vec<i32>) {
    heapify(nbs);
    let mut end = nbs.len() - 1;
    while end > 0 {
        nbs.swap(end, 0);
        end -= 1;
        sift_down(nbs, 0, end);
    }
}

/// put elements of the vec in heap order
fn heapify(nbs: &mut Vec<i32>) {
    let mut end = 1;
    let c = nbs.len();
    while end < c {
        sift_up(nbs, 0, end);
        end += 1;
    }
}

/// repair the heap from the root at start
fn sift_down(nbs: &mut [i32], start: usize, end: usize) {
    let mut root = start;
    while heap_left(root) <= end {
        let child = heap_left(root);
        let mut swp = root;
        if nbs[swp] < nbs[child] {
            swp = child;
        }
        if child + 1 < end && nbs[swp] < nbs[child + 1] {
            swp = child + 1;
        }
        if swp == root {
            return;
        } else {
            nbs.swap(root, swp);
            root = swp;
        }
    }
}

/// build the heap up
fn sift_up(nbs: &mut [i32], start: usize, end: usize) {
    let mut child = end;
    while child > start {
        let p = heap_parent(child);
        if nbs[p] < nbs[child] {
            nbs.swap(p, child);
            child = p;
        } else {
            return;
        }
    }
}

/// index of parent in heap
fn heap_parent(i: usize) -> usize {
    (i - 1) / 2
}

/// index of left child in heap
fn heap_left(i: usize) -> usize {
    2 * i + 1
}

/// index of right child in heap
#[allow(dead_code)]
fn heap_right(i: usize) -> usize {
    2 * i + 2
}

/// reverse in place
pub fn reverse(nbs: &mut Vec<i32>) {
    if !nbs.is_empty() {
        let mut i = 0;
        let mut j = nbs.len() - 1;
        while i < j {
            nbs.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

/// https://leetcode.com/problems/snapshot-array/
pub struct SnapshotArray {
    data: Vec<Vec<(i32, i32)>>,
    snap_id: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {
    pub fn new(length: i32) -> Self {
        SnapshotArray {
            data: vec![vec![]; length as usize],
            snap_id: 0,
        }
    }

    pub fn set(&mut self, index: i32, val: i32) {
        let e = &mut self.data[index as usize];
        if let Some(l) = e.last_mut() {
            if l.0 == self.snap_id {
                l.1 = val;
                return;
            }
        }
        e.push((self.snap_id, val));
    }

    pub fn snap(&mut self) -> i32 {
        self.snap_id += 1;
        self.snap_id - 1
    }

    pub fn get(&self, index: i32, snap_id: i32) -> i32 {
        let v = &self.data[index as usize];
        match v.binary_search_by_key(&snap_id, |a| a.0) {
            Ok(ix) => v[ix].1,
            Err(ix) if ix > 0 => v[ix - 1].1,
            _ => 0,
        }
    }
}

/// <https://leetcode.com/problems/k-radius-subarray-averages/>
pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let l = nums.len();
    let k = k as usize;
    let mut res = Vec::with_capacity(l);
    let range = (k * 2 + 1) as i64;
    let mut sum = None;
    for ix in 0..l {
        if ix < k || l - ix - 1 < k {
            res.push(-1);
        } else {
            let s: i64 = match sum {
                Some(old) => old - nums[ix - k - 1] as i64 + nums[ix + k] as i64,
                None => nums[ix - k..=ix + k].iter().map(|v| *v as i64).sum(),
            };
            sum = Some(s);
            res.push((s / range) as i32);
        }
    }
    res
}

/// <https://leetcode.com/problems/minimum-cost-to-make-array-equal/>
pub fn min_cost(nums: Vec<i32>, cost: Vec<i32>) -> i64 {
    if nums.is_empty() {
        return 0;
    }
    let mut min_cost = 0;
    let mut compound: Vec<(i64, i64)> = nums
        .into_iter()
        .zip(cost)
        .map(|(v, c)| (v as i64, c as i64))
        .collect();
    compound.sort_by_key(|t| t.0);
    let mut target = compound[0];
    let mut prefix = 0;
    let mut suffix = target.1;
    for (v, c) in compound.iter().skip(1) {
        min_cost += ((v - target.0).abs()) * c;
        suffix += c;
    }

    for (v, c) in compound.iter().skip(1) {
        let diff = (v - target.0).abs();
        prefix += target.1;
        suffix -= target.1;
        let cost = min_cost + (prefix * diff) - (suffix * diff);
        min_cost = min_cost.min(cost);

        target = (*v, *c);
    }
    min_cost
}

pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
    let mut dp = vec![vec![1; 1001]; nums.len()];
    let mut max = 0;
    for (ix, i) in nums.iter().enumerate().skip(1) {
        for (jx, j) in nums.iter().enumerate().take(ix) {
            let diff = (i - j + 500) as usize;
            let l = dp[jx][diff] + 1;
            dp[ix][diff] = l;
            max = max.max(l);
        }
    }
    max
}

/// <https://leetcode.com/problems/find-the-winner-of-an-array-game/>
pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
    let mut wins = 0;
    let mut winner_idx = 0;
    let mut next_idx = 1;

    loop {
        if winner_idx != next_idx {
            let i0 = arr[winner_idx];
            let i1 = arr[next_idx];
            if i0 > i1 {
                wins += 1;
            } else {
                wins = 1;
                winner_idx = next_idx;
            }
        }
        next_idx += 1;
        if next_idx == arr.len() {
            next_idx = 0;
        }

        if wins == k {
            return arr[winner_idx];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_number() {
        assert_eq!(None, missing_number(&[1, 2, 3], 1, 3));
        assert_eq!(None, missing_number(&[3, 2, 1], 1, 3));
        assert_eq!(Some(2), missing_number(&[1, 3], 1, 3));
        assert_eq!(Some(2), missing_number(&[1, 3, 1, 3], 1, 3));
        assert_eq!(Some(1), missing_number(&[2, 3], 1, 3));
        assert_eq!(Some(3), missing_number(&[1, 2], 1, 3));
        assert_eq!(Some(2), missing_number(&[3, 1], 1, 3));
        assert_eq!(Some(1), missing_number(&[3, 2], 1, 3));
        assert_eq!(Some(3), missing_number(&[2, 1], 1, 3));
        assert_eq!(
            Some(100),
            missing_number(&((1..100).collect::<Vec<i32>>()), 1, 100)
        );
    }

    #[test]
    #[should_panic(expected = "4 wasn't in range 1-3")]
    fn test_missing_number_panic() {
        assert_eq!(None, missing_number(&[1, 2, 3, 4], 1, 3));
    }

    #[test]
    fn test_duplicate() {
        assert_eq!(None, duplicate(&[1, 2, 3]));
        assert_eq!(Some(1), duplicate(&[1, 2, 3, 1]));
        assert_eq!(Some(1), duplicate(&[1, 1, 2, 3]));
        assert_eq!(Some(3), duplicate(&[1, 2, 3, 3]));
    }

    #[test]
    fn test_duplicates() {
        assert_eq!(IntSet::new(), duplicates(&[1, 2, 3]));
        assert_eq!(IntSet::from_vec(vec!(1)), duplicates(&[1, 2, 3, 1]));
        assert_eq!(IntSet::from_vec(vec!(1)), duplicates(&[1, 1, 2, 3]));
        assert_eq!(IntSet::from_vec(vec!(3)), duplicates(&[1, 2, 3, 3]));
        assert_eq!(IntSet::from_vec(vec!(1, 3)), duplicates(&[1, 1, 2, 3, 3]));
    }

    #[test]
    fn test_range() {
        assert_eq!((0, 0), range(&[0]));
        assert_eq!((0, 1), range(&[0, 1]));
        assert_eq!((-1, 1), range(&[0, 1, -1]));
    }

    #[test]
    #[should_panic(expected = "Empty array")]
    fn test_range_panic() {
        assert_eq!((0, 0), range(&[]));
    }

    #[test]
    fn test_pairs_product() {
        let e: Vec<(i32, i32)> = vec![];
        assert_eq!(e, pairs_product(&[], 2));
        assert_eq!(vec!((1, 2)), pairs_product(&[1, 2], 2));
        assert_eq!(vec!((2, 3)), pairs_product(&[1, 2, 3], 6));
        assert_eq!(vec!((2, 6), (3, 4)), pairs_product(&[1, 2, 3, 4, 6], 12));
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

    #[test]
    fn test_heap_sort() {
        test_sort(&heap_sort);
    }

    fn test_sort(f: &dyn Fn(&mut Vec<i32>)) {
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

    #[test]
    fn test_snapshot() {
        let mut snapshot = SnapshotArray::new(3); // set the length to be 3
        snapshot.set(0, 5); // Set array[0] = 5
        assert_eq!(0, snapshot.snap()); // Take a snapshot, return snap_id = 0
        assert_eq!(5, snapshot.get(0, 0));
        snapshot.set(0, 4);
        snapshot.set(0, 6);
        assert_eq!(5, snapshot.get(0, 0)); // Get the value of array[0] with snap_id = 0, return 5
        assert_eq!(6, snapshot.get(0, 1));
        assert_eq!(0, snapshot.get(1, 1));
        assert_eq!(0, snapshot.get(1, 0));
    }

    #[test]
    fn test_get_averages() {
        assert_eq!(
            vec![-1, -1, -1, 5, 4, 4, -1, -1, -1],
            get_averages(vec![7, 4, 3, 9, 1, 8, 5, 2, 6], 3)
        );
        assert_eq!(vec![100000], get_averages(vec![100000], 0));
        assert_eq!(vec![-1], get_averages(vec![8], 10000));
    }

    #[test]
    fn test_min_cost() {
        assert_eq!(8, min_cost(vec![1, 3, 5, 2], vec![2, 3, 1, 14]));
        assert_eq!(0, min_cost(vec![0, 0, 0, 0], vec![2, 3, 1, 14]));
    }

    #[test]
    fn test_longest_arith_seq_length() {
        assert_eq!(4, longest_arith_seq_length(vec![3, 6, 9, 12]));
        assert_eq!(3, longest_arith_seq_length(vec![9, 4, 7, 2, 10]));
        assert_eq!(4, longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8]));
    }

    #[test]
    fn test_winner() {
        assert_eq!(5, get_winner(vec![2, 1, 3, 5, 4, 6, 7], 2));
        assert_eq!(3, get_winner(vec![3, 2, 1], 10));
    }
}
