use pyo3::prelude::*;

mod types;
mod utils;
use crate::types::{CalamineError, CalamineSheet, CalamineWorkbook, CellValue};

#[pyfunction]
fn load_workbook(path_or_filelike: PyObject) -> PyResult<CalamineWorkbook> {
    CalamineWorkbook::from_object(path_or_filelike)
}

#[pymodule]
fn _python_calamine(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_workbook, m)?)?;
    m.add_class::<CalamineWorkbook>()?;
    m.add_class::<CalamineSheet>()?;
    m.add("CalamineError", py.get_type::<CalamineError>())?;
    Ok(())
}
