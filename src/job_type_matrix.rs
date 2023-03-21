use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use csv::ReaderBuilder;

fn read_csv() -> Result<(), Box<dyn Error>> {
    // Open the CSV file and create a CSV reader
    /*
    let mut file = File::open("../lib/matrix.csv")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut csv_reader = ReaderBuilder::new().has_headers(false).from_reader(contents.as_bytes());

    // Parse the CSV file into a vector of vectors
    let mut matrix = Vec::new();
    for result in csv_reader.records() {
        let row = result?;
        let row_vec: Vec<i32> = row.iter().map(|s| s.parse().unwrap()).collect();
        matrix.push(row_vec);
    }

    // Print the matrix
    for row in matrix.iter() {
        for &elem in row.iter() {
            print!("{:4} ", elem);
        }
        println!();
    }
*/
    Ok(())
}