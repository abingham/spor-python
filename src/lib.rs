use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};
use spor::alignment::align::Aligner;
use spor::alignment::smith_waterman::{SimpleScorer, SmithWaterman};

pub mod anchor;
mod fs_repository;

#[pyfunction]
fn align(a: &str, b: &str) -> PyResult<f32> {
    let scorer = SimpleScorer::default();
    let aligner = SmithWaterman::new(scorer);
    let (score, _) = aligner.align(a, b);
    Ok(score)
}

// TODO: Is it possible to define this in the anchor module? The macro stuff seems to prevent that right now.
/// anchor submodule
#[pymodule]
pub fn anchor(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<crate::anchor::PyContext>()?;
    m.add_class::<crate::anchor::PyAnchor>()?;
    Ok(())
}

// TODO: Can this go in fs_repository?
#[pyfunction]
pub fn initialize(path: &str) -> PyResult<()> {
    let path = std::path::Path::new(path);
    let future = spor::repository::fs_repository::initialize(path, None);
    futures::executor::block_on(future)
        .map_err(|err| pyo3::exceptions::OSError::py_err(format!("{}", err)))
}

/// Filesystem-based repository
#[pymodule]
pub fn fs_repository(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<crate::fs_repository::PyFSRepository>()?;
    m.add_wrapped(wrap_pyfunction!(initialize))?;
    Ok(())
}

/// Top-level spor module
#[pymodule]
fn spor(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(align))?;
    m.add_wrapped(wrap_pymodule!(anchor))?;
    m.add_wrapped(wrap_pymodule!(fs_repository))?;
    Ok(())
}
