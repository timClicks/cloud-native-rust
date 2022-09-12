use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("../swiss-mountains.csv")?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    for (i, line) in contents.lines().enumerate() {
        println!("{i}");
        if i > 2 {
            break;
        }

        println!("{:?}", line);
    }

    Ok(())
}