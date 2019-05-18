#![feature(never_type, proc_macro_hygiene, exhaustive_patterns)]
use std::path::Path;

use std::fs;
use std::io;

use serde_derive::Deserialize;

use powerset_enum::powerset_enum;

#[derive(Debug, err_derive::Error)]
#[error(display = "field is out of range")]
struct FieldError;

#[powerset_enum]
#[derive(Debug)]
enum Error {
    IoError(io::Error),
    JsonError(serde_json::Error),
    FieldError(FieldError),
}

#[derive(Debug, Default, Deserialize)]
struct Data {
    field: u32,
}

fn load_data_file_unchecked(path: &Path) -> Result<Data, Error![io::Error, serde_json::Error]> {
    let file = fs::File::open(path).map_err(Error::IoError)?;
    serde_json::from_reader(file).map_err(Error::JsonError)
}

fn load_data_file(path: &Path) -> Result<Data, Error![io::Error, serde_json::Error, FieldError]> {
    let file = fs::File::open(path).map_err(Error::IoError)?;
    let data: Data = serde_json::from_reader(file).map_err(Error::JsonError)?;
    if data.field > 100 {
        return Err(Error::FieldError(FieldError));
    }
    Ok(data)
}

fn load_data_file_or_default(path: &Path) -> Result<Data, Error![serde_json::Error, FieldError]> {
    load_data_file(path).or_else(|e| match e {
        Error::IoError(_) => Ok(Data::default()),
        Error::JsonError(e) => Err(Error::JsonError(e)),
        Error::FieldError(e) => Err(Error::FieldError(e)),
    })
}

fn load_data_best_effort(path: &Path) -> Data {
    load_data_file_or_default(path).unwrap_or_else(|e| match e {
        Error::JsonError(_) => {
            let content = fs::read_to_string(&path).expect("we managed read it before - why not now?");
            if let Some(m) = regex::Regex::new(r"\d+").unwrap().find(&content) {
                Data {
                    field: m.as_str().parse().unwrap(),
                }
            } else {
                Data::default()
            }
        },
        Error::FieldError(_) => load_data_file_unchecked(path).expect("we managed to read and parse it before - why not now?"),
    })
}

fn main() {
    let file_maker = powerset_enum_examples::FileMaker::new();
    let good_file = file_maker.make("good.json", r#"
    {
        "field": 42
    }
    "#);
    let bad_data = file_maker.make("bad-data.json", r#"
    {
        "field": 120
    }
    "#);
    let malformed = file_maker.make("malformed.json", r#"
    {
        "field": 23
    "#);

    println!("Good file: {:?}", load_data_best_effort(&good_file));
    println!("Bad data: {:?}", load_data_best_effort(&bad_data));
    println!("Malformed: {:?}", load_data_best_effort(&malformed));
    println!("Nonexistant: {:?}", load_data_best_effort(Path::new("nonexistant.json")));
}
