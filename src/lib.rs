use cpython::{py_fn, py_module_initializer, PyResult, Python};
use spor::alignment::align::Aligner;
use spor::alignment::smith_waterman::{SimpleScorer, SmithWaterman};

mod fs_repository;

fn align(_: Python, a: &str, b: &str) -> PyResult<f32> {
    let scorer = SimpleScorer::default();
    let aligner = SmithWaterman::new(scorer);
    let (score, _) = aligner.align(a, b);
    Ok(score)
}

py_module_initializer!(spor, |py, m| {
    m.add(py, "__doc__", "Anchored metadata.")?;
    m.add(py, "align", py_fn!(py, align(a: &str, b: &str)))?;
    m.add(py, "fs_repository", fs_repository::init_module(py)?)?;
    Ok(())
});
