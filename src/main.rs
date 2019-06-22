use interview::{get_words, longest_deranged};

fn main() {
    let r = get_words();
    match r {
        Ok(mut v) => {
            let od = longest_deranged(&mut v);
            match od {
                None => println!("No deranged anagrams found!"),
                Some((s1,s2)) => println!("{} {}",s1,s2),
            }
        },
        Err(e) => panic!("Could not read words: {}",e)
    } 
}
