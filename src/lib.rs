use pyo3::{prelude::*, types::PyDict};

/// Compute an UUID v7 using the current time
#[pyfunction]
fn uuid7() -> PyResult<Py<PyAny>> {
    let uuid = uuid::Uuid::now_v7();
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

/// UUID v7
#[pymodule]
fn oruuid(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(uuid7, m)?)?;
    Ok(())
}
