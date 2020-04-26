use pyo3::prelude::*;
use pyo3::{wrap_pymodule, wrap_pyfunction};
use spor::alignment::align::Aligner;
use spor::alignment::smith_waterman::{SimpleScorer, SmithWaterman};

pub mod anchor;
// mod fs_repository;

#[pyfunction]
fn align(a: &str, b: &str) -> PyResult<f32> {
    let scorer = SimpleScorer::default();
    let aligner = SmithWaterman::new(scorer);
    let (score, _) = aligner.align(a, b);
    Ok(score)
}

#[pymodule]
fn anchor(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<crate::anchor::PyContext>()?;
    Ok(())
}

#[pymodule]
/// A Python module implemented in Rust.
fn spor(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(align))?;

    m.add_wrapped(wrap_pymodule!(anchor))?;
    Ok(())
}
