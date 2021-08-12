use pyo3::prelude::*;
use std::io::Read;
use std::sync::Arc;
use std::sync::Mutex;
use pyo3::types::PyBytes;
use pyo3::PyObject;

fn open_read(path: &str) -> anyhow::Result<Arc<Mutex<dyn Read + Send>>> {
    let ret: Arc<Mutex<dyn Read + Send>> = Arc::new(Mutex::new(std::fs::File::open(path)?));
    Ok(ret)
}

#[pyclass]
struct PyRead {
    read: Arc<Mutex<dyn Read + Send>>
}

#[pymethods]
impl PyRead {
    fn read(&mut self) -> PyResult<PyObject> {
        Python::with_gil(|py| {
            let mut buf: Vec<u8> = Vec::new();
            self.read.lock().unwrap().read_to_end(&mut buf)?;
            Ok(PyBytes::new(py, &buf).into())
        })
    }
}

#[pyfunction]
fn py_open_read(path: &str) -> PyResult<PyRead> {
    match open_read(path) {
        Ok(value) => Ok(PyRead{read: value}),
        Err(err) => Err(pyo3::exceptions::PyOSError::new_err(format!(
            "open_read() failed: {}",
            err.to_string()
        ))),
    }
}

#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_open_read, m)?)?;

    Ok(())
}
