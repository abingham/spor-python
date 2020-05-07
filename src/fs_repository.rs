use crate::anchor::PyAnchor;
use pyo3::prelude::*;
use pyo3::types::{PyString, PyTuple};
use pyo3::PyIterProtocol;
use spor::repository::fs_repository::FSRepository;
use spor::repository::AnchorId;
use spor::repository::Repository;

#[pyclass(name=FSRepository, module="spor.repository.fs_repository")]
pub struct PyFSRepository {
    handle: FSRepository,
}

#[pymethods]
impl PyFSRepository {
    #[new]
    fn new(path: &str) -> PyResult<Self> {
        let path = std::path::Path::new(path);
        FSRepository::new(path, None)
            .map_err(|err| pyo3::exceptions::OSError::py_err(format!("{}", err)))
            .map(|repo| PyFSRepository { handle: repo })
    }

    #[getter]
    fn spor_dir(&self) -> PyResult<PyObject> {
        let path = self
            .handle
            .spor_dir()
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

    fn add(&self, anchor: &PyAnchor) -> PyResult<AnchorId> {
        let future = self.handle.add(anchor.handle.clone());
        futures::executor::block_on(future)
            .map_err(|err| pyo3::exceptions::RuntimeError::py_err(format!("{}", err)))
    }

    fn update(&self, anchor_id: AnchorId, anchor: &PyAnchor) -> PyResult<()> {
        let future = self.handle.update(anchor_id, &anchor.handle);
        futures::executor::block_on(future)
            .map_err(|err| pyo3::exceptions::RuntimeError::py_err(format!("{}", err)))
    }

    fn get(&self, anchor_id: AnchorId) -> PyResult<Option<PyAnchor>> {
        let f = self.handle.get(&anchor_id);
        futures::executor::block_on(f)
            .map_err(|err| pyo3::exceptions::RuntimeError::py_err(format!("{}", err)))
            .map(|opt| opt.map(|a| PyAnchor { handle: a }))
    }

    fn items(slf: PyRefMut<Self>) -> PyResult<ItemIterator> {
        let iter = ItemIterator {
            iter: slf.handle.iter(),
        };
        Ok(iter)
    }
}

#[pyproto]
impl PyIterProtocol for PyFSRepository {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Iterator> {
        let iter = Iterator {
            iter: slf.handle.iter(),
        };
        Ok(iter)
    }

    // TODO: Remove this! It's not necessary.
    //
    // This is neutered since we're only implementing half of the procotol (i.e. the "iterable" portion) here. pyo3
    // doesn't seem to have protocols for iterable and iterator, just iterator, so we're cheating a bit.
    // There's actually a trait default for this function, but it panics which seems less useful than raising
    // an exception.
    fn __next__(_slf: PyRefMut<Self>) -> PyResult<Option<PyObject>> {
        Err(pyo3::exceptions::TypeError::py_err(
            "PyFSRepository is not an iterator",
        ))
    }
}

/// Iterator over anchor-ids
#[pyclass]
pub struct Iterator {
    iter: spor::repository::fs_repository::RepositoryIterator,
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
    iter: spor::repository::fs_repository::RepositoryIterator,
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

