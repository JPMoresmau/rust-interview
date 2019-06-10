//! String functions

use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

/// are the two string anagrams of each other
pub fn anagram(s1: &str, s2: &str) -> bool {
    char_count(s1) == char_count(s2)
}

/// number of occurrences of the given character in the given string
pub fn char_occurrences(s :&str, c: &char) -> u32 {
    *char_count(s).get(c).unwrap_or(&0)
}

/// character count for the given string
fn char_count(s1: &str) -> HashMap<char,u32> {
    let mut ch1 = HashMap::new();
    for c in s1.chars() {
        let count = ch1.entry(c).or_insert(0);
        *count += 1;
    }
    ch1
}

/// reverse a string in place
pub fn reverse(s1: &mut String) {
    let mut v = vec!();
    while let Some(c) = s1.pop(){
        v.push(c);
    }
    for c in v.drain(..) {
        s1.push(c);
    }
}

/// check if a string is a palindrome
pub fn palindrome(s: &str) -> bool {
    // Rust doesn't want us indexing into characters, so let's brute force this
    let mut s2 = String::from(s);
    reverse(&mut s2);
    s == s2
}

/// all permutations of a string
pub fn permutations(s: &str) -> HashSet<String> {
    // swap the first character with every other character in the string
    // then ask for permutations for the remaining substring

    // naive version with loads of string allocations
    /*let mut v = HashSet::new();
    let mut cs : Vec<char> = s.chars().collect();
    v.insert(String::from(s));
    for i in 0..cs.len() {
        swap(&mut cs,0,i);
        let ns = String::from_iter(cs.iter());
        let s2 = permutations(&ns[1..]);
        for mut ns in s2 {
            ns.push(cs[0]);
            v.insert(ns);
        }
        
    }*/
    // hopefully less allocations, we clone only the vector
    let mut r = HashSet::new();
    permute(s.chars().collect(),0,&mut r);
    r.iter().map(|v| String::from_iter(v.iter())).collect()
}

/// internal permutation on character vector with start index
fn permute(mut v: Vec<char>, st: usize, s: &mut HashSet<Vec<char>>) {
    if st == v.len(){
        s.insert(v);
    } else {
        for i in st..v.len() {
            swap(&mut v,st,i);
            permute(v.clone(), st+1, s);
        }
    }
}

/// swap two characters
fn swap(nbs: &mut Vec<char>, low: usize, high: usize) {
    let tmp: char = nbs[low];
    nbs[low] = nbs[high];
    nbs[high] = tmp;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram(){
        assert!(anagram("hello","hello"));
        assert!(anagram("hello","holle"));
        assert!(anagram("",""));
        assert_eq!(false, anagram("hello", "hola"));
    }

    #[test]
    fn test_occurrences(){
        assert_eq!(1,char_occurrences("hello",&'h'));
        assert_eq!(2,char_occurrences("hello",&'l'));
        assert_eq!(0,char_occurrences("hello",&'a'));
    }

    #[test]
    fn test_reverse(){
        let mut s1=String::from("hello");
        reverse(&mut s1);
        assert_eq!("olleh",&s1);
    }

    #[test]
    fn test_palindrome(){
        assert!(palindrome("kayak"));
        assert_eq!(false,palindrome("hello"));
    }

    #[test]
    fn test_permutations(){
        let s= HashSet::from_iter(vec!("abc","bac","acb","cab","cba","bca").iter().map(|s| String::from(*s)));
        assert_eq!(s,permutations("abc"));
    }
}