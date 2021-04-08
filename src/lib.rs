use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Gets the type of a file from a byte stream.
///
/// Returns MIME as string.
#[pyfunction]
fn from_buffer(buf: &[u8]) -> Option<&'static str> {
    infer::get(buf).map(|x| x.mime_type())
}

/// Gets the type of a file from a filepath.
///
/// Does not look at file name or extension, just the contents.
/// Returns MIME as string
#[pyfunction]
fn from_file(path: &str) -> PyResult<Option<&'static str>> {
    infer::get_from_path(path)
        .map(|x| x.map(|t| t.mime_type()))
        .map_err(|e| pyo3::exceptions::PyOSError::new_err(e.to_string()))
}

/// rfiletype determines the MIME type a given file or byte stream
#[pymodule]
fn rfiletype(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_function(wrap_pyfunction!(from_buffer, m)?)?;
    m.add_function(wrap_pyfunction!(from_file, m)?)?;

    Ok(())
}
