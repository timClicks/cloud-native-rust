use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Year(u16);

impl FromStr for Year {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let year = match u16::from_str(s) {
            Ok(rank) => rank,
            Err(err) => return Err(format!("invalid year: {:?}", err)),
        };

        Ok(Year(year))
    }
}

// StringRecord(["Rank", "Mountain", "Height (m)", "Drop (m)", "Coordinates", "Range", "Canton(s)", "First\nascent"])

#[allow(dead_code)]
#[derive(Debug)]
struct Mountain {
    //rank,
    name: String,
    height: f32,
    // drop,
    // coordinates,
    // cantons,
    first_ascent: Option<Year>,
}

fn mountains() -> Result<Vec<Mountain>, Box<dyn Error>> {
    let file = File::open("../swiss-mountains.csv")?;
    let reader = BufReader::new(file);
    let mut reader = csv::Reader::from_reader(reader);
    
    let mut mountains = Vec::new();

    for result in reader.records() {
        let record = result?;

        let name = record.get(1).expect("no name for the mountain");
        let height = record.get(2).expect("no height");
        let first_ascent = match record.get(7) {
            Some(year) => Some(year.parse::<Year>()?),
            None => None,
        };

        let m = Mountain {
            name: name.to_string(),
            height: height.parse()?,
            first_ascent: first_ascent,
        };

        mountains.push(m);
    }

    Ok(mountains)
}

fn main() {
    match mountains() {
        Ok(mountains) => {
            for i in  0..3 {
                println!("{:?}", mountains[i]);
            }
        },
        Err(err) => {
            println!("encountered an error! {}", err);
            std::process::exit(1);
        },
    }
}
