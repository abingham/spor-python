use pyo3::prelude::*;
use pyo3::Python;

// TODO: Can we properly model the ownership relationship between Anchor and Context? Right now we just copy contexts
// when we need them from the anchor.

#[pyclass(name=Context, module="spor.anchor")]
pub struct PyContext {
    handle: spor::anchor::Context,
}

#[pymethods]
impl PyContext {
    #[new]
    fn new(text: &str, offset: usize, width: usize, context_width: usize) -> PyResult<Self> {
        spor::anchor::Context::new(text, offset, width, context_width)
            .or_else(|err| Err(pyo3::exceptions::ValueError::py_err(format!("{}", err))))
            .map(|context| PyContext { handle: context })
    }

    #[getter]
    fn before(&self) -> PyResult<String> {
        Ok(self.handle.before().clone())
    }

    #[getter]
    fn offset(&self) -> PyResult<usize> {
        Ok(self.handle.offset())
    }

    #[getter]
    fn topic(&self) -> PyResult<String> {
        Ok(self.handle.topic().clone())
    }

    #[getter]
    fn after(&self) -> PyResult<String> {
        Ok(self.handle.after().clone())
    }

    #[getter]
    fn width(&self) -> PyResult<usize> {
        Ok(self.handle.width())
    }

    #[getter]
    fn full_text(&self) -> PyResult<String> {
        Ok(self.handle.full_text())
    }
}

#[pyclass(name=Anchor, module="spor.anchor")]
pub struct PyAnchor {
    handle: spor::anchor::Anchor,
}

#[pymethods]
impl PyAnchor {
    #[new]
    fn new(
        file_path: String,
        context: &PyContext,
        metadata: PyObject,
        encoding: String,
    ) -> PyResult<Self> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let file_path = std::path::Path::new(&file_path);

        let metadata = PyModule::import(py, "yaml")
            .and_then(|yaml| yaml.call("dump", (metadata,), None))
            .and_then(|string| string.extract::<String>())
            .and_then(|string| {
                serde_yaml::from_str::<serde_yaml::Value>(&string)
                    .or_else(|err| Err(pyo3::exceptions::ValueError::py_err(format!("{}", err))))
            })?;

        spor::anchor::Anchor::new(file_path, context.handle.clone(), metadata, encoding)
            .or_else(|err| Err(pyo3::exceptions::OSError::py_err(format!("{}", err))))
            .map(|anchor| PyAnchor { handle: anchor })
    }

    #[getter]
    fn file_path(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let path =
            self.handle
                .file_path()
                .to_str()
                .ok_or(pyo3::exceptions::RuntimeError::py_err(
                    "Unable to convert path to string",
                ))?;

        PyModule::import(py, "pathlib")
            .and_then(|pathlib| pathlib.call("Path", (path,), None))
            .map(|path| path.to_object(py))
    }

    #[getter]
    fn context(&self) -> PyResult<PyContext> {
        Ok(PyContext {
            handle: self.handle.context().clone(),
        })
    }

    #[getter]
    fn encoding(&self) -> PyResult<String> {
        Ok(self.handle.encoding().clone())
    }

    #[getter]
    fn metadata(&self) -> PyResult<PyObject> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let string = serde_yaml::to_string(self.handle.metadata())
            .or_else(|err| Err(pyo3::exceptions::ValueError::py_err(format!("{}", err))))?;

        PyModule::import(py, "yaml")
            .and_then(|module| module.call("safe_load", (string,), None))
            .map(|result| result.to_object(py))
    }
}
