use pyo3::prelude::*;
mod utils;

#[pymodule]
mod conda_share {
    use pyo3::exceptions::PyRuntimeError;
    use pyo3::prelude::*;

    use crate::utils::py_root_dir;
    use conda_share as core;

    use std::collections::HashMap;
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
        #[pyo3(get)]
        version_requirements: Option<String>,
    }

    impl From<core::CondaPackage> for PyCondaPackage {
        fn from(p: core::CondaPackage) -> Self {
            Self {
                name: p.name,
                version: p.version,
                build: p.build,
                channel: p.channel,
                version_requirements: p.version_requirements,
            }
        }
    }

    #[pymethods]
    impl PyCondaPackage {
        fn __repr__(&self) -> String {
            format!(
                "CondaPackage(name={}, version={}, build={}, channel={}, version_requirements={})",
                self.name,
                self.version.as_deref().unwrap_or("n/a"),
                self.build.as_deref().unwrap_or("n/a"),
                self.channel.as_deref().unwrap_or("n/a"),
                self.version_requirements.as_deref().unwrap_or("n/a"),
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

    #[pyclass(module = "conda_share")]
    #[derive(Clone)]
    struct PyCondaInfo {
        #[pyo3(get)]
        gid: u64,
        #[pyo3(get)]
        uid: u64,
        #[pyo3(get)]
        active_prefix: Option<String>,
        #[pyo3(get)]
        active_prefix_name: Option<String>,
        #[pyo3(get)]
        av_data_dir: String,
        #[pyo3(get)]
        av_metadata_url_base: Option<String>,
        #[pyo3(get)]
        channels: Vec<String>,
        #[pyo3(get)]
        conda_build_version: String,
        #[pyo3(get)]
        conda_env_version: String,
        #[pyo3(get)]
        conda_location: String,
        #[pyo3(get)]
        conda_prefix: String,
        #[pyo3(get)]
        conda_shlvl: i64,
        #[pyo3(get)]
        conda_version: String,
        #[pyo3(get)]
        default_prefix: String,
        #[pyo3(get)]
        env_vars: HashMap<String, String>,
        #[pyo3(get)]
        envs: Vec<String>,
        #[pyo3(get)]
        envs_dirs: Vec<String>,
        #[pyo3(get)]
        netrc_file: Option<String>,
        #[pyo3(get)]
        offline: bool,
        #[pyo3(get)]
        platform: String,
        #[pyo3(get)]
        python_version: String,
        #[pyo3(get)]
        rc_path: String,
        #[pyo3(get)]
        requests_version: String,
        #[pyo3(get)]
        root_prefix: String,
        #[pyo3(get)]
        root_writable: bool,
        #[pyo3(get)]
        site_dirs: Vec<String>,
        #[pyo3(get)]
        sys_executable: String,
        #[pyo3(get)]
        sys_prefix: String,
        #[pyo3(get)]
        sys_version: String,
        #[pyo3(get)]
        sys_rc_path: String,
        #[pyo3(get)]
        user_agent: String,
        #[pyo3(get)]
        user_rc_path: String,
        #[pyo3(get)]
        virtual_pkgs: Vec<Vec<String>>,
    }

    impl From<core::CondaInfo> for PyCondaInfo {
        fn from(info: core::CondaInfo) -> Self {
            Self {
                gid: info.gid,
                uid: info.uid,
                active_prefix: info.active_prefix,
                active_prefix_name: info.active_prefix_name,
                av_data_dir: info.av_data_dir,
                av_metadata_url_base: info.av_metadata_url_base,
                channels: info.channels,
                conda_build_version: info.conda_build_version,
                conda_env_version: info.conda_env_version,
                conda_location: info.conda_location,
                conda_prefix: info.conda_prefix,
                conda_shlvl: info.conda_shlvl,
                conda_version: info.conda_version,
                default_prefix: info.default_prefix,
                env_vars: info.env_vars,
                envs: info.envs,
                envs_dirs: info.envs_dirs,
                netrc_file: info.netrc_file,
                offline: info.offline,
                platform: info.platform,
                python_version: info.python_version,
                rc_path: info.rc_path,
                requests_version: info.requests_version,
                root_prefix: info.root_prefix,
                root_writable: info.root_writable,
                site_dirs: info.site_dirs,
                sys_executable: info.sys_executable,
                sys_prefix: info.sys_prefix,
                sys_version: info.sys_version,
                sys_rc_path: info.sys_rc_path,
                user_agent: info.user_agent,
                user_rc_path: info.user_rc_path,
                virtual_pkgs: info.virtual_pkgs,
            }
        }
    }

    #[pyfunction]
    fn conda_env_export(env_name: &str, from_history: bool) -> PyResult<PyCondaEnv> {
        let env = core::conda_env_export(env_name, from_history).map_err(to_py_err)?;
        Ok(PyCondaEnv { inner: env })
    }

    #[pyfunction]
    fn conda_list(env_name: &str) -> PyResult<Vec<PyCondaPackage>> {
        let pkgs = core::conda_list(env_name).map_err(to_py_err)?;
        Ok(pkgs.into_iter().map(PyCondaPackage::from).collect())
    }

    #[pyfunction]
    fn conda_env_list() -> PyResult<Vec<String>> {
        core::conda_env_list().map_err(to_py_err)
    }

    #[pyfunction]
    fn conda_info() -> PyResult<PyCondaInfo> {
        let info = core::conda_info().map_err(to_py_err)?;
        Ok(PyCondaInfo::from(info))
    }

    #[pyfunction]
    fn env_exists(env_name: &str) -> PyResult<bool> {
        core::env_exists(env_name).map_err(to_py_err)
    }

    #[pyfunction]
    fn current_env() -> PyResult<String> {
        match core::current_env().map_err(to_py_err)? {
            Some(env) => Ok(env),
            None => Err(PyRuntimeError::new_err(
                "No conda environment is currently active.",
            )),
        }
    }

    #[pyfunction]
    fn share_env(env_name: &str) -> PyResult<PyCondaEnv> {
        let env = core::share_env(env_name).map_err(to_py_err)?;
        Ok(PyCondaEnv { inner: env })
    }

    #[pyfunction]
    fn share_current_env() -> PyResult<PyCondaEnv> {
        let env = core::share_current_env().map_err(to_py_err)?;
        Ok(PyCondaEnv { inner: env })
    }

    #[pyfunction]
    #[pyo3(signature = (path=None))]
    fn save_current_env(py: Python<'_>, path: Option<&str>) -> PyResult<()> {
        let env = core::share_current_env().map_err(to_py_err)?;
        let path = match path {
            Some(p) => PathBuf::from(p),
            None => py_root_dir(py)?.join(env.name.clone() + ".yml"),
        };
        env.save(path).map_err(to_py_err)
    }
}
