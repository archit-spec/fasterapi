use pyo3::prelude::*;
use std::fs::File;
use std::io::{self, Read};



#[pyfunction]
fn read_from_file(file_path: &str) -> PyResult<String, io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Locate the start and end index of `sum_as_string` function content
    let start_index = contents.find("#[pyfunction]\nfn sum_as_string(a: usize, b: usize) -> PyResult<String> {") 
                        .ok_or(io::Error::new(io::ErrorKind::Other, "Function not found"))?;
    let end_index = contents[start_index..].find("}").ok_or(io::Error::new(io::ErrorKind::Other, "Function not terminated"))? + start_index;

    // Extract `sum_as_string` function content
    let extracted_function = contents[start_index..=end_index].to_string();
    Ok(extracted_function)
}



/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}



#[pymodule]
fn string_sum(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(read_from_file, m)?)?;

    Ok(())
}
