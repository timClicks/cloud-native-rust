use std::error::Error;
use std::io::BufReader;
use std::fs::File;

fn mountains() -> Result<(), Box<dyn Error>> {
    let file = File::open("../swiss-mountains.csv")?;
    let reader = BufReader::new(file);
    let mut reader = csv::Reader::from_reader(reader);
    
    for (i, result) in reader.records().enumerate() {
        if i > 2 {
            break;
        }
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn main() {
    if let Err(err) = mountains() {
        println!("encountered an error! {}", err);
        std::process::exit(1);
    }
}
