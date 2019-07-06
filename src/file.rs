//! File operations
//! Submitted to http://www.rosettacode.org/wiki/Append_a_record_to_the_end_of_a_text_file#Rust
 
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Result;
use std::io::Write;
use std::path::Path;

/// Password record with all fields
#[derive(Eq, PartialEq, Debug)]
pub struct PasswordRecord {
    pub account: String,
    pub password: String,
    pub uid: u64,
    pub gid: u64,
    pub gecos: Vec<String>,
    pub directory: String,
    pub shell: String,
}


impl PasswordRecord {
    /// new instance, cloning all fields
    pub fn new(
        account: &str,
        password: &str,
        uid: u64,
        gid: u64,
        gecos: Vec<&str>,
        directory: &str,
        shell: &str,
    ) -> PasswordRecord {
        PasswordRecord {
            account: account.to_string(),
            password: password.to_string(),
            uid,
            gid,
            gecos: gecos.iter().map(|s| s.to_string()).collect(),
            directory: directory.to_string(),
            shell: shell.to_string(),
        }
    }

    /// convert to one line string
    pub fn to_line(&self) -> String {
        let gecos = self.gecos.join(",");
        format!(
            "{}:{}:{}:{}:{}:{}:{}",
            self.account, self.password, self.uid, self.gid, gecos, self.directory, self.shell
        )
    }

    /// read record from line
    pub fn from_line(line: &str) -> PasswordRecord {
        let sp: Vec<&str> = line.split(":").collect();
        if sp.len() < 7 {
            panic!("Less than 7 fields found");
        } else {
            let uid = sp[2].parse().expect("Cannot parse uid");
            let gid = sp[3].parse().expect("Cannot parse gid");
            let gecos = sp[4].split(",").collect();
            PasswordRecord::new(sp[0], sp[1], uid, gid, gecos, sp[5], sp[6])
        }
    }
}

/// read all records from file
pub fn read_password_file(file_name: &str) -> Result<Vec<PasswordRecord>> {
    let p = Path::new(file_name);
    if !p.exists() {
        Ok(vec![])
    } else {
        let f = OpenOptions::new().read(true).open(p)?;
        Ok(BufReader::new(&f)
            .lines()
            .map(|l| PasswordRecord::from_line(&l.unwrap()))
            .collect())
    }
}

/// overwrite file with records
pub fn overwrite_password_file(file_name: &str, recs: &Vec<PasswordRecord>) -> Result<()> {
    let f = OpenOptions::new()
        .create(true)
        .write(true)
        .open(file_name)?;
    write_records(f, recs)
}

/// append records to file
pub fn append_password_file(file_name: &str, recs: &Vec<PasswordRecord>) -> Result<()> {
    let f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_name)?;
    write_records(f, recs)
}

/// internal, write records line by line
fn write_records(f: File, recs: &Vec<PasswordRecord>) -> Result<()> {
    let mut writer = BufWriter::new(f);
    for rec in recs {
        write!(writer, "{}\n", rec.to_line())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_password_file() {
        fs::remove_file("passwd").unwrap_or(());
        let recs0 = read_password_file("passwd").expect("cannot read file");
        assert_eq!(0, recs0.len());
        let mut recs1 = vec![
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
        assert_eq!(2, recs2.len());
        assert_eq!(recs1, recs2);
        overwrite_password_file("passwd", &recs1).expect("cannot write file");
        let recs3 = read_password_file("passwd").expect("cannot read file");
        assert_eq!(2, recs3.len());
        assert_eq!(recs1, recs3);
        let mut append0 = vec![PasswordRecord::new(
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
        let mut full = vec![];
        full.append(&mut recs1);
        full.append(&mut append0);
        let recs4 = read_password_file("passwd").expect("cannot read file");
        assert_eq!(3, recs4.len());
        assert_eq!(full, recs4);
        fs::remove_file("passwd").expect("cannot delete file");
    }
}
