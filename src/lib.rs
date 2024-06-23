use pyo3::{prelude::*, types::PyDict};

fn to_python(uuid: uuid::Uuid) -> PyResult<Py<PyAny>> {
    Python::with_gil(|py| {
        let kwargs = PyDict::new_bound(py);
        kwargs.set_item("int", uuid.as_u128())?;
        let pyuuid = PyModule::import_bound(py, "uuid")?;
        pyuuid
            .getattr("UUID")?
            .call((), Some(&kwargs))
            .map(|u| u.unbind())
    })
}

#[pyfunction]
fn uuid1() -> PyResult<Py<PyAny>> {
    Python::with_gil(|py| {
        let pyuuid = PyModule::import_bound(py, "uuid")?;
        pyuuid.getattr("uuid1")?.call0().map(|u| u.unbind())
    })
}

#[pyfunction]
fn uuid4() -> PyResult<Py<PyAny>> {
    Python::with_gil(|py| {
        let pyuuid = PyModule::import_bound(py, "uuid")?;
        pyuuid.getattr("uuid4")?.call0().map(|u| u.unbind())
    })
}

#[pyfunction]
fn uuid4r() -> PyResult<Py<PyAny>> {
    let v4 = uuid::Uuid::new_v4();
    to_python(v4)
}

#[pyfunction]
fn uuid7() -> PyResult<Py<PyAny>> {
    let v7 = uuid::Uuid::now_v7();
    to_python(v7)
}

/// A Python module implemented in Rust.
#[pymodule]
fn oruuid(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(uuid1, m)?)?;
    m.add_function(wrap_pyfunction!(uuid4, m)?)?;
    m.add_function(wrap_pyfunction!(uuid4r, m)?)?;
    m.add_function(wrap_pyfunction!(uuid7, m)?)?;
    Ok(())
}
