/*use interview::{get_words, longest_deranged};

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
}*/
/*
use interview::pi;

fn main() {
    println!("pi(7): {}", pi(7));
}*/

use interview::*;

fn main() {
    let recs1 = vec![
        PasswordRecord::new(
            "jsmith",
            "x",
            1001,
            1000,
            vec![
                "Joe Smith",
                "Room 1007",
                "(234)555-8917",
                "(234)555-0077",
                "jsmith@rosettacode.org",
            ],
            "/home/jsmith",
            "/bin/bash",
        ),
        PasswordRecord::new(
            "jdoe",
            "x",
            1002,
            1000,
            vec![
                "Jane Doe",
                "Room 1004",
                "(234)555-8914",
                "(234)555-0044",
                "jdoe@rosettacode.org",
            ],
            "/home/jdoe",
            "/bin/bash",
        ),
    ];

    overwrite_password_file("passwd", &recs1).expect("cannot write file");
    let recs2 = read_password_file("passwd").expect("cannot read file");
    println!("Original file:");
    for r in recs2 {
        println!("{}", r.to_line());
    }
    let append0 = vec![PasswordRecord::new(
        "xyz",
        "x",
        1003,
        1000,
        vec![
            "X Yz",
            "Room 1003",
            "(234)555-8913",
            "(234)555-0033",
            "xyz@rosettacode.org",
        ],
        "/home/xyz",
        "/bin/bash",
    )];
    append_password_file("passwd", &append0).expect("cannot append to file");
    let recs2 = read_password_file("passwd").expect("cannot read file");
    println!();
    println!("Appended file:");
    for r in recs2 {
        println!("{}", r.to_line());
    }
}
