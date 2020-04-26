
use pyo3::prelude::*;
use spor::repository::Repository;
use crate::anchor::PyAnchor;
use spor::repository::AnchorId;
use spor::repository::fs_repository::FSRepository;

#[pyclass(name=FSRepository, module="spor.repository.fs_repository")]
pub struct PyFSRepository {
    handle: FSRepository
}

#[pymethods]
impl PyFSRepository {
    #[new]
    fn new(path: &str) -> PyResult<Self> {
        let path = std::path::Path::new(path);
        FSRepository::new(path, None)
            .map_err(|err| {
                pyo3::exceptions::OSError::py_err(format!("{}", err))
            })
            .map(|repo| {
                PyFSRepository { handle: repo }
            })
    }

    #[getter]
    fn spor_dir(&self) -> PyResult<PyObject> {
          let path = self.handle.spor_dir().to_str()
             .ok_or(pyo3::exceptions::ValueError::py_err("Unable to convert path to string"))
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

    // TODO: iteration 
}

// TODO: initialize()

// pub fn init_module(py: Python) -> PyResult<PyModule> {
//     let m = PyModule::new(py, "fs_repository")?;
//     m.add(py, "__doc__", "Filesystem-based repository")?;
//     m.add_class::<FSRepository>(py)?;
//     // initialize
//     Ok(m)
// }
