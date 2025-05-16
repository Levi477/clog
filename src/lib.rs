use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::path::PathBuf;
mod backend;
use backend::main;

/// Adds a new user
#[pyfunction]
fn add_new_user(password: &str, clogfile_path: &str) {
    let path = PathBuf::from(clogfile_path);
    main::add_new_user(&path, password);
}

/// Adds a folder
#[pyfunction]
fn add_folder(clogfile_path: &str, password: &str) -> PyResult<()> {
    let path = PathBuf::from(clogfile_path);
    main::add_folder(&path, password).map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e))
}

/// Edits a file
#[pyfunction]
fn edit_file(
    password: &str,
    clogfile_path: &str,
    filename: &str,
    foldername: &str,
    new_file_content: &str,
) {
    let path = PathBuf::from(clogfile_path);
    main::edit_file(password, &path, filename, foldername, new_file_content);
}

/// Decrypt file
#[pyfunction]
fn get_file_content(
    password: &str,
    clogfile_path: &str,
    filename: &str,
    foldername: &str,
) -> String {
    let path = PathBuf::from(clogfile_path);
    main::get_file_content(&path, filename, foldername, password)
}

/// Adds a file
#[pyfunction]
fn add_file(
    password: &str,
    clogfile_path: &str,
    filename: &str,
    foldername: &str,
    file_content: &str,
) -> PyResult<()> {
    let path = PathBuf::from(clogfile_path);
    main::add_file(password, &path, filename, foldername, file_content)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(e))
}

/// Daily check and update
#[pyfunction]
fn daily_check_and_update_metadata(clogfile_path: &str, password: &str) {
    let path = PathBuf::from(clogfile_path);
    main::daily_check_and_update_metadata(&path, password);
}

/// Get Metadata in json_serialized
#[pyfunction]
fn get_clean_metadata(password: &str, clogfile_path: &str) -> String {
    let path = PathBuf::from(clogfile_path);
    main::get_clean_metadata(password, &path)
}

/// PyO3 modu#[pymodule]
#[pymodule]
fn clog(_py: Python, m: Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add_new_user, &m)?)?;
    m.add_function(wrap_pyfunction!(add_folder, &m)?)?;
    m.add_function(wrap_pyfunction!(edit_file, &m)?)?;
    m.add_function(wrap_pyfunction!(get_clean_metadata, &m)?)?;
    m.add_function(wrap_pyfunction!(get_file_content, &m)?)?;
    m.add_function(wrap_pyfunction!(add_file, &m)?)?;
    m.add_function(wrap_pyfunction!(daily_check_and_update_metadata, &m)?)?;
    Ok(())
}
