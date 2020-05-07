use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};
use spor::alignment::align::Aligner;
use spor::alignment::smith_waterman::{SimpleScorer, SmithWaterman};

pub mod anchor;
mod fs_repository;

use crate::anchor::PyInit_anchor;
use crate::fs_repository::PyInit_fs_repository;

#[pyfunction]
fn align(a: &str, b: &str) -> PyResult<f32> {
    let scorer = SimpleScorer::default();
    let aligner = SmithWaterman::new(scorer);
    let (score, _) = aligner.align(a, b);
    Ok(score)
}

/// Top-level spor module
#[pymodule]
fn spor(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(align))?;
    m.add_wrapped(wrap_pymodule!(anchor))?;
    m.add_wrapped(wrap_pymodule!(fs_repository))?;
    Ok(())
}
