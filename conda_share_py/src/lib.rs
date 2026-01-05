use pyo3::prelude::*;

#[pymodule]
mod conda_share {
    use pyo3::exceptions::PyRuntimeError;
    use pyo3::prelude::*;
    
    use conda_share as core;

    use std::path::PathBuf;

    fn to_py_err(e: core::CondaError) -> PyErr {
        PyRuntimeError::new_err(e.to_string())
    }

    #[pyclass(module = "conda_share")]
    #[derive(Clone)]
    struct PyCondaPackage {
        #[pyo3(get)]
        name: String,
        #[pyo3(get)]
        version: Option<String>,
        #[pyo3(get)]
        build: Option<String>,
        #[pyo3(get)]
        channel: Option<String>,
    }

    impl From<core::CondaPackage> for PyCondaPackage {
        fn from(p: core::CondaPackage) -> Self {
            Self {
                name: p.name,
                version: p.version,
                build: p.build,
                channel: p.channel,
            }
        }
    }

    #[pymethods]
    impl PyCondaPackage {
        fn __repr__(&self) -> String {
            format!(
                "CondaPackage(name={}, version={}, build={}, channel={})",
                self.name,
                self.version.as_deref().unwrap_or("n/a"),
                self.build.as_deref().unwrap_or("n/a"),
                self.channel.as_deref().unwrap_or("n/a"),
            )
        }
    }   

    #[pyclass(module = "conda_share")]
    #[derive(Clone)]
    struct PyCondaEnv {
        inner: core::CondaEnv,
    }

    #[pymethods]
    impl PyCondaEnv {
        #[getter]
        fn name(&self) -> &str {
            &self.inner.name
        }

        #[getter]
        fn channels(&self) -> &Vec<String> {
            &self.inner.channels
        }

        #[getter]
        fn conda_deps(&self) -> Vec<PyCondaPackage> {
            self.inner
                .conda_deps
                .clone()
                .into_iter()
                .map(PyCondaPackage::from)
                .collect()
        }

        #[getter]
        fn pip_deps(&self) -> Vec<PyCondaPackage> {
            self.inner
                .pip_deps
                .clone()
                .into_iter()
                .map(PyCondaPackage::from)
                .collect()
        }

        fn to_yaml(&self) -> PyResult<String> {
            self.inner.to_yaml().map_err(to_py_err)
        }

        fn save(&self, path: &str) -> PyResult<()> {
            let path = PathBuf::from(path);
            self.inner.save(&path).map_err(to_py_err)
        }

        fn __repr__(&self) -> String {
            self.inner.to_string()
        }
    }

    // ----- module functions -----

    #[pyfunction]
    fn conda_env_list() -> PyResult<Vec<String>> {
        core::conda_env_list().map_err(to_py_err)
    }

    #[pyfunction]
    fn env_exists(env_name: &str) -> PyResult<bool> {
        core::env_exists(env_name).map_err(to_py_err)
    }

    #[pyfunction]
    fn conda_list(env_name: &str) -> PyResult<Vec<PyCondaPackage>> {
        let pkgs = core::conda_list(env_name).map_err(to_py_err)?;
        Ok(pkgs.into_iter().map(PyCondaPackage::from).collect())
    }

    #[pyfunction]
    fn conda_env_export(env_name: &str, from_history: bool) -> PyResult<PyCondaEnv> {
        let env = core::conda_env_export(env_name, from_history).map_err(to_py_err)?;
        Ok(PyCondaEnv { inner: env })
    }

    #[pyfunction]
    fn sharable_env(env_name: &str) -> PyResult<PyCondaEnv> {
        let env = core::sharable_env(env_name).map_err(to_py_err)?;
        Ok(PyCondaEnv { inner: env })
    }
}