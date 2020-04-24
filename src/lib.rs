use cpython::{PyResult, Python, py_module_initializer, py_fn};
use spor::alignment::smith_waterman::{SimpleScorer, SmithWaterman};
use spor::alignment::align::Aligner;

fn align(_: Python, a: &str, b: &str) -> PyResult<f32> {
    let scorer = SimpleScorer::default();
    let aligner = SmithWaterman::new(scorer);
    let (score, _) = aligner.align(a, b);
    Ok(score)
}

py_module_initializer!(spor, |py, m| {
    m.add(py, "__doc__", "Anchored metadata.")?;
    m.add(py, "align", py_fn!(py, align(a: &str, b: &str)))?;
    Ok(())
});

