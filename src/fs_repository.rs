use cpython::{py_class, PyErr, PyModule, PyResult, Python};

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
    // def half(&self) -> PyResult<i32> {
    //     println!("half() was called with self={:?}", self.number(py));
    //     Ok(self.number(py) / 2)
    // }

    data repo: spor::repository::fs_repository::FSRepository;
});

pub fn init_module(py: Python) -> PyResult<PyModule> {
    let m = PyModule::new(py, "fs_repository")?;
    m.add(py, "__doc__", "Filesystem-based repository")?;
    m.add_class::<FSRepository>(py)?;
    Ok(m)
}
