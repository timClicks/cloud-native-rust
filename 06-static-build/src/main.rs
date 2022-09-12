use std::cmp::Ordering;
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

#[derive(Debug)]
struct Mountain {
    name: String,
    height: f32,
    // drop: f32,
    // coordinates: String, // TODO: parse into custom type
    // cantons: Vec<String>,
    first_ascent: Option<Year>,
}

struct Mountains {
    inner: Vec<Mountain>,
}

impl From<Vec<Mountain>> for Mountains {
    fn from(data: Vec<Mountain>) -> Self {
        Mountains { inner: data }
    }
}

impl Mountains {
    fn by_name(&mut self) {
        self.inner.sort_by(|a, b| {
            (a.name).cmp(&b.name) 
        })
    }

    fn by_height(&mut self) {
        self.inner.sort_by(|a, b| {
            // reverse order so highest is first
            (b.height).partial_cmp(&a.height).unwrap() // Should never be NaN/infinity/etc
        })
    }

    fn by_height_reversed(&mut self) {
        self.inner.sort_by(|a, b| {
            (a.height).partial_cmp(&b.height).unwrap() // Should never be NaN/infinity/etc
        })
    }

    fn by_first_ascent(&mut self) {
        self.inner.sort_by(|a, b| {
            match (a.first_ascent.as_ref(), b.first_ascent.as_ref()) {
                (None, None) => Ordering::Equal,
                (Some(year_a), Some(year_b)) => year_a.cmp(year_b),
                (_, None) => Ordering::Less,
                (None, _) => Ordering::Greater,
            }
        })
    }
}

fn load_mountains() -> Result<Mountains, Box<dyn Error>> {
    let file = File::open("../swiss-mountains.csv")?;
    let reader = BufReader::new(file);
    let mut reader = csv::Reader::from_reader(reader);
    
    let mut mountains = Vec::new();

    for result in reader.records() {
        let record = result?;

        let name = record.get(1).expect("no name for the mountain");
        let height = record.get(2).expect("no height");
        let first_ascent = match record.get(7) {
            Some(raw_year) => match raw_year.parse::<Year>() {
                Ok(year) => Some(year),
                Err(_) => None,
            },
            None => None,
        };

        let m = Mountain {
            name: name.to_string(),
            height: height.parse()?,
            first_ascent: first_ascent,
        };

        mountains.push(m);
    }

    Ok(Mountains::from(mountains))
}

fn main() {
    let mut mountains = match load_mountains() {
        Ok(mountains) => mountains,
        Err(err) => {
            println!("encountered an error! {}", err);
            std::process::exit(1);
        },
    };

    let args: Vec<_> = std::env::args().collect();

    match args.iter().nth(1).unwrap_or(&String::from("")).as_str() {
        "ascent" | "first-ascent" => mountains.by_first_ascent(),
        "name" => mountains.by_name(),
        "height" => mountains.by_height(),
        "tallest-first" => mountains.by_height(),
        "smallest-first" => mountains.by_height_reversed(),
        _ => (),
    };

    let n_rows: usize = match args.iter().nth(2) {
        None => 3,
        Some(n) => n.parse::<usize>().unwrap_or(3),
    };

    for i in  0..n_rows {
        println!("{:?}", mountains.inner[i]);
    }
}
