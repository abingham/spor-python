use cpython::ObjectProtocol;
use cpython::{py_class, PyErr, PyModule, PyObject, PyResult, Python};
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

    def add(&self, anchor: crate::anchor::Anchor) -> PyResult<spor::repository::AnchorId> {
        let f = anchor.anchor(py);
        // let f = self.repo(py).add(anchor);
        // futures::executor::block_on(f)
        //     .or_else(|err| Err(PyErr::new::<cpython::exc::RuntimeError, _>(py, err)))
        Err(PyErr::new::<cpython::exc::RuntimeError, _>(py, "fnord"))
    }

    // def update(&self, anchor_id: String, anchor: spor::anchor::Anchor) -> PyResult<()> {
    //     let f = self.repo(py).update(anchor_id, &anchor);
    //     futures::executor::block_on(f)
    //         .or_else(|err| Err(PyErr::new::<cpython::exc::RuntimeError, _>(py, err)))
    // }

    // def get(&self, anchor_id: spor::repository::AnchorId) -> PyResult<Option<spor::anchor::Anchor>> {
    //     let f = self.repo(py).get(&anchor_id);
    //     futures::executor::block_on(f)
    //         .or_else(|err| Err(PyErr::new::<cpython::exc::RuntimeError, _>(py, err)))
    // }

    data repo: spor::repository::fs_repository::FSRepository;
});

pub fn init_module(py: Python) -> PyResult<PyModule> {
    let m = PyModule::new(py, "fs_repository")?;
    m.add(py, "__doc__", "Filesystem-based repository")?;
    m.add_class::<FSRepository>(py)?;
    // initialize
    Ok(m)
}
