use crate::parse::CLFile;
use crate::parse::CLRecord;
use csv::{Reader, ReaderBuilder};

const TEST_FILE_PATH: &str = "data/bourbon_k7_south.cl";

#[test]
fn can_open_file() {
    let _reader = Reader::from_path(TEST_FILE_PATH);
}

#[test]
fn can_read_file() {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(TEST_FILE_PATH)
        .expect("failed to open CSV file");
    for result in rdr.records() {
        println!("record: {:?}", result.expect("failed to read record"));
    }
}

#[test]
fn can_parse_file() {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(TEST_FILE_PATH)
        .expect("failed to open CSV file");
    for result in rdr.deserialize::<CLRecord>() {
        match result {
            Ok(record) => println!("{:?}", record),
            Err(e) => panic!("failed to deserialize record: {}", e),
        }
    }
}

#[test]
fn can_make_new_cel_file() {
    let cl_file = CLFile::create_new(TEST_FILE_PATH);
    println!("{:?}", cl_file);
}

#[test]
fn can_load() {
    let cl_file = CLFile::create_new(TEST_FILE_PATH).unwrap();
    println!("{:?}", cl_file);
}

// #[test]
// fn can_open_and_re_save() {
//     let mut cl_file = CLFile::new(TEST_FILE_PATH);
//     cl_file.load().unwrap();
//     cl_file.pathbuf = PathBuf::from("")?;
//
//
//
//
//
//
//     let cl_file = CLFile::new("");
//     cl_file.save().unwrap();
// }
