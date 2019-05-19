use std::path::Path;

use std::fs;
use std::io;

use serde_derive::Deserialize;

#[derive(Debug, err_derive::Error)]
#[error(display = "field is out of range")]
struct FieldError;

#[derive(Debug, err_derive::Error)]
enum Error {
    #[error(display = "{}", _0)]
    IoError(io::Error),
    #[error(display = "{}", _0)]
    JsonError(serde_json::Error),
    #[error(display = "{}", _0)]
    FieldError(FieldError),
}

#[derive(Debug, Default, Deserialize)]
struct Data {
    field: u32,
}

fn load_data_file_unchecked(path: &Path) -> Result<Data, Error> {
    let file = fs::File::open(path).map_err(Error::IoError)?;
    serde_json::from_reader(file).map_err(Error::JsonError)
}

fn load_data_file(path: &Path) -> Result<Data, Error> {
    let data = load_data_file_unchecked(&path)?;
    if data.field < 20 {
        return Err(Error::FieldError(FieldError));
    }
    Ok(data)
}

fn load_data_file_or_default(path: &Path) -> Result<Data, Error> {
    load_data_file(path).or_else(|e| match e {
        Error::IoError(_) => Ok(Data::default()),
        e => Err(e),
    })
}

fn load_data_best_effort(path: &Path) -> Data {
    load_data_file_or_default(path).unwrap_or_else(|e| match e {
        Error::IoError(e) => panic!("IO error should have been impossible, but we got {}", e),
        Error::JsonError(_) => {
            let content =
                fs::read_to_string(&path).expect("we managed read it before - why not now?");
            if let Some(m) = regex::Regex::new(r"\d+").unwrap().find(&content) {
                Data {
                    field: m.as_str().parse().unwrap(),
                }
            } else {
                Data::default()
            }
        }
        Error::FieldError(_) => load_data_file_unchecked(path)
            .expect("we managed to read and parse it before - why not now?"),
    })
}

fn main() {
    let tempdir = tempfile::tempdir().unwrap();
    let good_file = tempdir.path().join("good.json");
    fs::write(&good_file, r#"
    {
        "field": 42
    }
    "#).unwrap();
    let bad_data = tempdir.path().join("bad-data.json");
    fs::write(&bad_data, r#"
    {
        "field": 120
    }
    "#).unwrap();
    let malformed = tempdir.path().join("malformed.json");
    fs::write(&malformed, r#"
    {
        "field": 23
    "#).unwrap();

    println!("Good file: {:?}", load_data_best_effort(&good_file));
    println!("Bad data: {:?}", load_data_best_effort(&bad_data));
    println!("Malformed: {:?}", load_data_best_effort(&malformed));
    println!("Nonexistant: {:?}", load_data_best_effort(Path::new("nonexistant.json")));
}
