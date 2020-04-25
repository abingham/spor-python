use cpython::{py_class, PyErr, PyModule, PyResult, Python};

py_class!(class Context |py| {
    def __new__(_cls, text: &str, offset: usize, width: usize, context_width: usize) -> PyResult<Context> {
        spor::anchor::Context::new(text, offset, width, context_width)
            .or_else(|err| {
                Err(PyErr::new::<cpython::exc::ValueError, _>(py, format!("{}", err)))
            })
            .and_then(|context| {
                Context::create_instance(py, context)
            })
    }

    def before(&self) -> PyResult<String> {
        Ok(self.context(py).before().clone())
    }

    def offset(&self) -> PyResult<usize> {
        Ok(self.context(py).offset())
    }

    def topic(&self) -> PyResult<String> {
        Ok(self.context(py).topic().clone())
    }

    def after(&self) -> PyResult<String> {
        Ok(self.context(py).after().clone())
    }

    def width(&self) -> PyResult<usize> {
        Ok(self.context(py).width())
    }

    def full_text(&self) -> PyResult<String> {
        Ok(self.context(py).full_text())
    }

    data context: spor::anchor::Context;
});

pub fn init_module(py: Python) -> PyResult<PyModule> {
    let m = PyModule::new(py, "anchor")?;
    m.add(py, "__doc__", "Anchor implementation")?;
    m.add_class::<Context>(py)?;
    Ok(m)
}
