const TEST_FILE_A: &str = "data/bb_k7_south.cl";
const TEST_FILE_B: &str = "data/bb_k7_south_copy.cl";

use crate::centerline::file::CLRecord;

/// These tests don't really assess anything local in the project.
/// Instead, they test the tools that local modules depend on.
/// A Failed test in this module likly indicates something
/// configured incorrectly, rather than an error in the local source.
#[test]
fn can_open_file() {
    let _reader = csv::Reader::from_path(TEST_FILE_A);
}
#[test]
fn can_read_file_with_csv_crate() {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(TEST_FILE_A)
        .expect(&format!("cant make a reader from {}", TEST_FILE_A));
    for result in rdr.records() {
        println!("record: {:?}", result.expect("failed to read record"));
    }
}
#[test]
fn can_parse_file_with_csv_crate() {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(TEST_FILE_A)
        .expect("failed to open CSV file");
    for result in rdr.deserialize::<CLRecord>() {
        match result {
            Ok(record) => println!("{:?}", record),
            Err(e) => panic!("failed to deserialize record: {}", e),
        }
    }
}
#[test]
fn can_create_new_files() {
    std::fs::File::create(TEST_FILE_B).unwrap();
    std::fs::remove_file(TEST_FILE_B).unwrap();
}
