use pyo3::class::PyIterProtocol;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::types::PyType;
use pyo3::PyObject;
use std::io::BufRead;
use std::io::Read;
use std::io::Write;
use std::ops::DerefMut;
use std::sync::Arc;
use std::sync::Mutex;

fn open_read(path: &str) -> anyhow::Result<Arc<Mutex<dyn Read + Send>>> {
    let ret: Arc<Mutex<dyn Read + Send>> = Arc::new(Mutex::new(std::fs::File::open(path)?));
    Ok(ret)
}

fn open_write(path: &str) -> anyhow::Result<Arc<Mutex<dyn Write + Send>>> {
    let ret: Arc<Mutex<dyn Write + Send>> = Arc::new(Mutex::new(std::fs::File::create(path)?));
    Ok(ret)
}

#[pyclass]
struct PyReadIter {
    inner: std::vec::IntoIter<String>,
}

#[pyproto]
impl PyIterProtocol for PyReadIter {
    fn __iter__(slf: PyRef<Self>) -> PyRef<Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<Self>) -> Option<PyObject> {
        let string: String;
        match slf.inner.next() {
            Some(value) => string = value,
            None => {
                return None;
            }
        };
        let buf: Vec<u8> = string.into_bytes();
        Python::with_gil(|py| Some(PyBytes::new(py, &buf).into()))
    }
}

#[pyclass]
struct PyRead {
    read: Arc<Mutex<dyn Read + Send>>,
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

    fn close(&mut self) -> PyResult<()> {
        Ok(())
    }

    fn __enter__(&self) -> Self {
        let read = self.read.clone();
        PyRead { read }
    }

    fn __exit__(
        &mut self,
        _ty: Option<&PyType>,
        _value: Option<&PyAny>,
        _traceback: Option<&PyAny>,
    ) -> bool {
        true
    }
}

#[pyproto]
impl PyIterProtocol for PyRead {
    fn __iter__(slf: PyRef<Self>) -> PyResult<Py<PyReadIter>> {
        let mut guard = slf.read.lock().unwrap();
        let mut reader = std::io::BufReader::new(guard.deref_mut());
        let mut lines: Vec<String> = Vec::new();
        loop {
            let mut line = String::new();
            if let Ok(len) = reader.read_line(&mut line) {
                if len == 0 {
                    break;
                }
                lines.push(line);
                continue;
            }
            break;
        }
        let iter = PyReadIter {
            inner: lines.clone().into_iter(),
        };
        Py::new(slf.py(), iter)
    }
}

#[pyclass]
struct PyWrite {
    write: Arc<Mutex<dyn Write + Send>>,
}

#[pymethods]
impl PyWrite {
    fn write(&mut self, buf: &[u8]) -> PyResult<usize> {
        let mut guard = self.write.lock().unwrap();
        match guard.write_all(buf) {
            Ok(_) => Ok(buf.len()),
            Err(err) => Err(pyo3::exceptions::PyOSError::new_err(format!(
                "write() failed: {}",
                err.to_string()
            ))),
        }
    }

    fn close(&mut self) -> PyResult<()> {
        Ok(())
    }

    fn __enter__(&self) -> Self {
        let write = self.write.clone();
        PyWrite { write }
    }

    fn __exit__(
        &mut self,
        _ty: Option<&PyType>,
        _value: Option<&PyAny>,
        _traceback: Option<&PyAny>,
    ) -> bool {
        true
    }
}

#[pyfunction]
fn py_open_read(path: &str) -> PyResult<PyRead> {
    match open_read(path) {
        Ok(value) => Ok(PyRead { read: value }),
        Err(err) => Err(pyo3::exceptions::PyOSError::new_err(format!(
            "open_read() failed: {}",
            err.to_string()
        ))),
    }
}

#[pyfunction]
fn py_open_write(path: &str) -> PyResult<PyWrite> {
    match open_write(path) {
        Ok(value) => Ok(PyWrite { write: value }),
        Err(err) => Err(pyo3::exceptions::PyOSError::new_err(format!(
            "open_write() failed: {}",
            err.to_string()
        ))),
    }
}

#[pymodule]
fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_open_read, m)?)?;
    m.add_function(wrap_pyfunction!(py_open_write, m)?)?;

    Ok(())
}
