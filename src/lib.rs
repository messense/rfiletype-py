use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// File type
#[pyclass(module = "rfiletype")]
#[derive(FromPyObject)]
struct Type {
    /// file mime type
    #[pyo3(get)]
    mime_type: String,
    /// file extension
    #[pyo3(get)]
    extension: String,
}

impl From<infer::Type> for Type {
    fn from(ty: infer::Type) -> Self {
        Type {
            mime_type: ty.mime_type().to_string(),
            extension: ty.extension().to_string(),
        }
    }
}

/// Gets the type of a file from a byte stream.
///
/// Returns MIME as string.
#[pyfunction]
fn from_buffer(py: Python, buf: &[u8]) -> Option<Type> {
    py.allow_threads(|| infer::get(buf).map(|x| x.into()))
}

/// Gets the type of a file from a filepath.
///
/// Does not look at file name or extension, just the contents.
/// Returns MIME as string
#[pyfunction]
fn from_file(py: Python, path: &str) -> PyResult<Option<Type>> {
    let res = py.allow_threads(|| infer::get_from_path(path).map(|x| x.map(|t| t.into())));
    res.map_err(|e| pyo3::exceptions::PyOSError::new_err(e.to_string()))
}

/// rfiletype determines the MIME type a given file or byte stream
#[pymodule]
fn rfiletype(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<Type>()?;
    m.add_function(wrap_pyfunction!(from_buffer, m)?)?;
    m.add_function(wrap_pyfunction!(from_file, m)?)?;

    Ok(())
}
