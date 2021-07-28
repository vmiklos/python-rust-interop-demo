#![deny(warnings)]
#![warn(clippy::all)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

//! The rust module allows Python <-> Rust interop.

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyFloat;
use chrono::NaiveDateTime;
use chrono::Utc;

trait Timer {
    fn time(&self) -> i64;
}

fn string_today(timer: &dyn Timer) -> String {
    let now = timer.time();
    let date_time = NaiveDateTime::from_timestamp(now, 0);
    date_time.format("%Y-%m-%d").to_string()
}

struct ChronoTimer {
}

impl Timer for ChronoTimer {
    fn time(&self) -> i64 {
        let now = Utc::now();
        now.timestamp()
    }
}

struct PyTimer<'a> {
    timer: &'a PyAny,
}

impl<'a> PyTimer<'a> {
    fn new(timer:  &'a PyAny) -> Self {
        PyTimer{timer}
    }

    fn time_or_err(&self) -> PyResult<i64> {
        let now_any = self.timer.call_method0("time")?;
        Ok(now_any.downcast::<PyFloat>()?.value().round() as i64)
    }
}

impl<'a> Timer for PyTimer<'a> {
    fn time(&self) -> i64 {
        if let Ok(value) = self.time_or_err() {
            return value;
        }

        0
    }
}

#[pyfunction]
fn py_string_today(timer: &PyAny) -> PyResult<String> {
    let timer = PyTimer::new(timer);
    Ok(string_today(&timer))
}

#[pyclass]
struct PyChronoTimer {
    chrono_timer: ChronoTimer,
}

#[pymethods]
impl PyChronoTimer {
    #[new]
    fn new() -> Self {
        let chrono_timer = ChronoTimer{};
        PyChronoTimer{chrono_timer}
    }

    fn time(&self) -> f64 {
        self.chrono_timer.time() as f64
    }
}

#[pymodule]
fn rust(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_string_today, m)?)?;
    m.add_class::<PyChronoTimer>()?;

    Ok(())
}
