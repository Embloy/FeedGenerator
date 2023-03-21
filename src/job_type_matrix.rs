use std::{error::Error, io, process};
use csv::StringRecord;

// Main program entry point function
pub(crate)fn build_matrix() -> Result<(), Box<dyn Error>> {
    // Reader from path of test csv file
    let reader = csv::Reader::from_path("./lib/matrix.csv");
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    // Iterate through each record (line) in the CSV
    for record in reader?.records() {
        // Check that the string record actually exists
        let string_record = record?;
        // Convert the StringRecord to a Vec<i32>
        let int_values: Vec<i32> = string_record
            .iter()
            .map(|s| s.parse::<i32>().unwrap_or(0))
            .collect();
        matrix.push(int_values);
    }
    Ok(())
}