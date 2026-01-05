use std::path::PathBuf;

use pyo3::exceptions::PyRuntimeError;
use pyo3::ffi::c_str;
use pyo3::prelude::*;
use pyo3::types::PyDict;

pub fn py_root_dir(py: Python<'_>) -> PyResult<PathBuf> {
    let locals = PyDict::new(py);

    py.run(
        c_str!("from pathlib import Path; ROOT = Path(__file__).resolve().parent if \"__file__\" in globals() else Path.cwd()"),
        None,
        Some(&locals),
    )?;

    let root = locals
        .get_item("ROOT")?
        .ok_or_else(|| PyRuntimeError::new_err("ROOT not set by python snippet"))?;

    root.extract::<PathBuf>()
}
