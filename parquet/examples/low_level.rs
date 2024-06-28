use std::fs::File;

use parquet::file::reader::{FileReader, SerializedFileReader};

pub fn main() {
    let testdata = arrow::util::test_util::parquet_test_data();
    let path = format!("{testdata}/alltypes_plain.parquet");
    let file = File::open(path).unwrap();

    let reader = SerializedFileReader::new(file).unwrap();

    println!("Num of row groups {}", reader.num_row_groups());

    let parquet_metadata = reader.metadata();

    println!("parquet_metadata: {:#?}", parquet_metadata);
}
