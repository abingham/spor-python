use cpython::ObjectProtocol;
use cpython::{py_class, PyDict, PyErr, PyModule, PyResult, Python};

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

py_class!(class Anchor |py| {
    def __new__(_cls,
        file_path: String,
        context: Context,
        metadata: PyDict,
        encoding: String
    ) -> PyResult<Anchor> {
        let file_path = std::path::Path::new(&file_path);
        let metadata = PyModule::import(py, "yaml")
            .and_then(|yaml| yaml.get(py, "dump"))
            .and_then(|dump| dump.call(py, (metadata,), None))
            .and_then(|string| string.extract::<String>(py))
            .and_then(|string|
                serde_yaml::from_str::<serde_yaml::Value>(&string)
                    .or_else(|err|
                        Err(PyErr::new::<cpython::exc::ValueError, _>(py, format!("{}", err)))))?;

        let c = context.context(py);
        let context = spor::anchor::Context::new(&c.full_text(), c.offset(), c.topic().len(), c.width())
            .or_else(|err| {
                Err(PyErr::new::<cpython::exc::ValueError, _>(py, format!("{}", err)))
            })?;

        spor::anchor::Anchor::new(file_path, context, metadata, encoding)
            .or_else(|err| {
                Err(PyErr::new::<cpython::exc::OSError, _>(py, format!("{}", err)))
            })
            .and_then(|anchor| {
                Anchor::create_instance(py, anchor)
            })
    }

    def file_path(&self) -> PyResult<String> {
        let p = self.anchor(py).file_path();
        p.to_str()
            .ok_or(PyErr::new::<cpython::exc::ValueError, _>(py, 
                "Unable to convert path to string"))
            .map(|s| s.to_owned())
    }

    def encoding(&self) -> PyResult<String> {
        Ok(self.anchor(py).encoding().clone())
    }

    // pub fn context(&self) -> &Context {
    //     return &self.context;
    // }

    def metadata(&self) -> PyResult<PyDict> {
        let metadata = self.anchor(py).metadata();

        let metadata_string = serde_yaml::to_string(metadata)
            .or_else(|err| 
                Err(PyErr::new::<cpython::exc::ValueError, _>(py, format!("{}", err))))?;

        PyModule::import(py, "yaml")
            .and_then(|yaml| yaml.get(py, "safe_load"))
            .and_then(|load| load.call(py, (metadata_string,), None))
            .and_then(|dict| 
                dict.cast_into::<PyDict>(py)
                    .or_else(|err| Err(PyErr::from(err))))
    }

    data anchor: spor::anchor::Anchor;
});

pub fn init_module(py: Python) -> PyResult<PyModule> {
    let m = PyModule::new(py, "anchor")?;
    m.add(py, "__doc__", "Anchor implementation")?;
    m.add_class::<Anchor>(py)?;
    m.add_class::<Context>(py)?;
    Ok(m)
}
