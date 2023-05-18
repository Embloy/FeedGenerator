use std::error::Error;

use smartcore::linalg::basic::matrix::DenseMatrix;

//MATRIX is defined outside the build function to make it accessible by query
static mut MATRIX: Option<Vec<Vec<f64>>> = None;
static mut MATRIX_2:Option<DenseMatrix<f64>> = None;
pub static mut _REDUCED_MATRIX:Option<DenseMatrix<f64>> = None;



// build should be called when server is started (-> main.rs)
pub(crate) fn build() -> Result<(), Box<dyn Error>> {
    // Reader from path of test csv file
    let reader = csv::Reader::from_path("./lib/matrix.csv");
    let mut matrix: Vec<Vec<f64>> = Vec::new();
    // Iterate through each record (line) in the CSV
    for record in reader?.records() {
        // Check that the string record actually exists
        let string_record = record?;
        // Convert the StringRecord to a Vec<f64>
        let int_values: Vec<f64> = string_record
            .iter()
            .map(|s| s.parse::<f64>().unwrap_or(0.0))
            .collect();
        matrix.push(int_values);
    }
    unsafe {
        MATRIX = Some(matrix);
        MATRIX_2 = Some(DenseMatrix::from_2d_vec(&MATRIX.as_ref().unwrap()));
    }
    Ok(())
}

pub(crate) fn query(row: f64, column: f64) -> f64 {
    let matrix = unsafe { MATRIX.as_ref().unwrap() };
    //println!("Row {}{:?} and Column {} and Cell {:?}", row,matrix[row as usize], column, matrix[row as usize][column as usize]);
    // Return the value at the specified column and row
    // index is required to be usize in rust (-> i was told this by an AI though...)
    matrix[row as usize][column as usize]
}
/*
//NOTE: THIS PCA IS JUST A TEST HOW IT CAN BE DONE IN RUST. IT TOTALLY DOES NOT MAKE SENSE TO MAKE PCA ON THE MATRIX!!!
fn pca() {
    unsafe {
        let pca = PCA::fit(MATRIX_2.as_ref().unwrap(), PCAParameters::default().with_n_components(2)).unwrap(); // Reduce number of features to 2
        REDUCED_MATRIX = Some(pca.transform(MATRIX_2.as_ref().unwrap()).unwrap());
    }
}
*/
