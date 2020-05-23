use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};
use spor::alignment::align::Aligner;
use spor::alignment::smith_waterman::{SimpleScorer, SmithWaterman};

pub mod anchor;
mod repository;

use crate::anchor::PyInit_anchor;
use crate::repository::PyInit_repository;

#[pyfunction]
fn align(a: &str, b: &str) -> PyResult<f32> {
    let scorer = SimpleScorer::default();
    let aligner = SmithWaterman::new(scorer);
    let alignments = aligner.align(a, b);
    Ok(alignments.score())
}

/// Top-level spor module
#[pymodule]
fn spor(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(align))?;
    m.add_wrapped(wrap_pymodule!(anchor))?;
    m.add_wrapped(wrap_pymodule!(repository))?;
    Ok(())
}
