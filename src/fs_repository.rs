use cpython::ObjectProtocol;
use cpython::{py_class, PyErr, PyInt, PyTuple, PyModule, PyObject, PyResult, Python, ToPyObject};
use spor::repository::Repository;
use crate::anchor::Anchor;

py_class!(class FSRepository |py| {
    def __new__(_cls, path: &str) -> PyResult<FSRepository> {
        let path = std::path::Path::new(path);
        spor::repository::fs_repository::FSRepository::new(path, None)
            .or_else(|err| {
                Err(PyErr::new::<cpython::exc::OSError, _>(py, format!("{}", err)))
            })
            .and_then(|repo| {
                FSRepository::create_instance(py, repo)
            })
    }

    def spor_dir(&self) -> PyResult<PyObject> {
        let path = self.repo(py).spor_dir().to_str()
             .ok_or(PyErr::new::<cpython::exc::ValueError, _>(py,
                 "Unable to convert path to string"))
             .map(|s| s.to_owned())?;

        PyModule::import(py, "pathlib")
            .and_then(|pathlib| pathlib.get(py, "Path"))
            .and_then(|ctor| ctor.call(py, (path,), None))
    }
    // def iteration
    
    def add(&self, anchor: spor::anchor::Anchor) -> PyResult<spor::anchor::AnchorId> {
        let f = self.repo(py).add(anchor);
        futures::executor::block_on(f)
            .or_else(|err| Err(PyErr::new::<cpython::exc::RuntimeError, _>(py, err)))
    }
    def update(&self, anchor_id: String, anchor: spor::anchor::Anchor) -> PyResult<()> {
        let f = self.repo(py).update(anchor_id, &anchor);
        futures::executor::block_on(f)
            .or_else(|err| Err(PyErr::new::<cpython::exc::RuntimeError, _>(py, err)))

    }
    // def get

    data repo: spor::repository::fs_repository::FSRepository;
});

pub fn init_module(py: Python) -> PyResult<PyModule> {
    let m = PyModule::new(py, "fs_repository")?;
    m.add(py, "__doc__", "Filesystem-based repository")?;
    m.add_class::<FSRepository>(py)?;
    // initialize
    Ok(m)
}
