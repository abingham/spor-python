use crate::anchor::PyAnchor;
use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple};
use pyo3::wrap_pyfunction;
use pyo3::PyIterProtocol;
use spor::repository;

#[pyclass(name=Repository)]
pub struct PyRepository {
    handle: repository::Repository,
}

#[pymethods]
impl PyRepository {
    #[new]
    fn new(path: &str) -> PyResult<Self> {
        let path = std::path::Path::new(path);
        repository::open(path, None)
            .map_err(|err| pyo3::exceptions::OSError::py_err(format!("{}", err)))
            .map(|repo| PyRepository { handle: repo })
    }

    #[getter]
    fn repo_dir(&self) -> PyResult<PyObject> {
        let path = self
            .handle
            .repo_dir()
            .to_str()
            .ok_or(pyo3::exceptions::ValueError::py_err(
                "Unable to convert path to string",
            ))
            .map(|s| s.to_owned())?;

        let gil = Python::acquire_gil();
        let py = gil.python();

        PyModule::import(py, "pathlib")
            .and_then(|pathlib| pathlib.call("Path", (path,), None))
            .map(|path| path.to_object(py))
    }

    fn add(&self, anchor: &PyAnchor) -> PyResult<repository::AnchorId> {
        self.handle.add(&anchor.handle)
            .map_err(|err| pyo3::exceptions::RuntimeError::py_err(format!("{}", err)))
    }

    fn update(&self, anchor_id: repository::AnchorId, anchor: &PyAnchor) -> PyResult<()> {
        self.handle.update(&anchor_id, &anchor.handle)
            .map_err(|err| pyo3::exceptions::RuntimeError::py_err(format!("{}", err)))
    }

    fn get(&self, anchor_id: repository::AnchorId) -> PyResult<PyAnchor> {
        self.handle.get(&anchor_id)
            .map_err(|err| pyo3::exceptions::RuntimeError::py_err(format!("{}", err)))
            .map(|a| PyAnchor { handle: a })
    }

    fn items(slf: PyRefMut<Self>) -> PyResult<ItemIterator> {
        let iter = ItemIterator {
            iter: slf.handle.clone().into_iter(),
        };
        Ok(iter)
    }
}

#[pyproto]
impl PyIterProtocol for PyRepository {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Iterator> {
        let iter = Iterator {
            iter: slf.handle.clone().into_iter(),
        };
        Ok(iter)
    }
}

/// Iterator over anchor-ids
#[pyclass]
pub struct Iterator {
    iter: repository::iteration::Iterator,
}

#[pyproto]
impl PyIterProtocol for Iterator {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<Iterator>> {
        Ok(slf.into())
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<PyObject>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let result = slf
            .iter
            .next()
            .map(|(id, _anchor)| PyString::new(py, &id).to_object(py));

        Ok(result)
    }
}

/// Iterator over (id, anchor) tuples suitable for e.g. the items() method.
#[pyclass]
pub struct ItemIterator {
    iter: repository::iteration::Iterator,
}

#[pyproto]
impl PyIterProtocol for ItemIterator {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<ItemIterator>> {
        Ok(slf.into())
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<PyObject>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let result = slf.iter.next().map(|(id, anchor)| {
            let objects: Vec<PyObject> = vec![
                PyString::new(py, &id).to_object(py),
                PyAnchor { handle: anchor }.into_py(py),
            ];

            PyTuple::new(py, objects).to_object(py)
        });

        Ok(result)
    }
}

#[pyfunction]
pub fn initialize(path: &str) -> PyResult<()> {
    let path = std::path::Path::new(path);
    spor::repository::initialize(path, None)
        .map_err(|err| pyo3::exceptions::OSError::py_err(format!("{}", err)))
}

/// Filesystem-based repository
#[pymodule]
pub fn repository(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<crate::repository::PyRepository>()?;
    m.add_wrapped(wrap_pyfunction!(initialize))?;
    Ok(())
}
