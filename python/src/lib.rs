use cel2db;
use pyo3::prelude::*;

#[pyfunction]
fn cel_to_db(cel_expr: &str, sql_dialect: &str) -> PyResult<String> {
    Ok(cel2db::cel_to_db(cel_expr, sql_dialect).unwrap())
}

#[pymodule]
fn cel2db_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cel_to_db, m)?)?;
    Ok(())
}
