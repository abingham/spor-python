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
}

#[pyproto]
impl PyIterProtocol for PyFSRepository {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<FSRepositoryIterator> {
        let iter = FSRepositoryIterator { iter: slf.handle.iter() };
        Ok(iter)
    }

    // This is neutered since we're only implementing half of the procotol (i.e. the "iterable" portion) here. pyo3
    // doesn't seem to have protocols for iterable and iterator, just iterator, so we're cheating a bit.
    fn __next__(_slf: PyRefMut<Self>) -> PyResult<Option<PyObject>> {
        Err(pyo3::exceptions::TypeError::py_err("PyFSRepository is not an iterator"))
    }
}

#[pyclass]
pub struct FSRepositoryIterator {
    iter: spor::repository::fs_repository::RepositoryIterator,
}

#[pyproto]
impl PyIterProtocol for FSRepositoryIterator {
    fn __iter__(slf: PyRefMut<Self>) -> PyResult<Py<FSRepositoryIterator>> {
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

// TODO: initialize()

// pub fn init_module(py: Python) -> PyResult<PyModule> {
//     let m = PyModule::new(py, "fs_repository")?;
//     m.add(py, "__doc__", "Filesystem-based repository")?;
//     m.add_class::<FSRepository>(py)?;
//     // initialize
//     Ok(m)
// }
