use std::{error::Error, io, process};
use csv::StringRecord;

//MATRIX is defined outside the build function to make it accessible by query
static mut MATRIX: Option<Vec<Vec<i32>>> = None;

// build should be called when server is started (-> main.rs)
pub(crate)fn build() -> Result<(), Box<dyn Error>> {
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
    unsafe {
        MATRIX = Some(matrix);
    }
    Ok(())
}

pub(crate)fn query(column: i32, row: i32) -> i32 {
    let matrix = unsafe { MATRIX.as_ref().unwrap() };
    // Return the value at the specified column and row
    // index is required to be usize in rust (-> i was told this by an AI though...)
    matrix[row as usize][column as usize]
}