use std::io::Write;
use std::path::Path;
use std::process::{Command, Output};
use std::ffi::OsStr;
use std::fs::File;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CondaError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Missing version for package")]
    MissingVersion,
    #[error("Environment '{0}' does not exist. Available environments: {1}")]
    EnvNotFound(String, String),
    #[error("Conda command failed (conda {0}): {1}")]
    CondaCommandFailed(String, String),
    #[error("Failed to execute conda command. This likely means it can't find the 'conda' executable in your PATH. {0}")]
    CommandExecutionFailed(String),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CondaEnv {
    name: String,
    channels: Vec<String>,
    conda_deps: Vec<CondaPackage>,
    pip_deps: Vec<CondaPackage>,
}

impl CondaEnv {
    pub fn to_yaml(&self) -> Result<String, CondaError> {
        let mut yml = String::new();
        yml.push_str(&format!("name: {}\n", self.name));

        yml.push_str("channels:\n");
        yml.extend(self.channels.iter().map(|c| format!("  - {}\n", c)));
        if !self.conda_deps.is_empty() {
            yml.push_str("dependencies:\n");
            for dep in &self.conda_deps {
                let version = dep.version.clone().ok_or(CondaError::MissingVersion)?;
                yml.push_str(&format!("  - {}={}\n", dep.name, version));
            }
        }

        if !self.pip_deps.is_empty() {
            yml.push_str("  - pip:\n");
            for dep in &self.pip_deps {
                let version = dep.version.clone().ok_or(CondaError::MissingVersion)?;
                yml.push_str(&format!("      - {}=={}\n", dep.name, version));
            }
        }

        Ok(yml)
    }
    pub fn save(&self, path: &Path) -> Result<(), CondaError> {
        let yml = self.to_yaml()?;
        let mut file = File::create(path)?;
        file.write_all(yml.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CondaPackage {
    name: String,
    version: Option<String>,
    build: Option<String>,
    channel: Option<String>,
}

pub fn env_exists(env_name: &str) -> Result<bool, CondaError> {
    let available_envs = conda_env_list()?;
    Ok(available_envs.contains(&env_name.to_string()))
}

pub fn sharable_env(env_name: &str) -> Result<CondaEnv, CondaError> {
    let conda_env_from_history = conda_env_export(env_name, true)?;
    let conda_env_export = conda_env_export(env_name, false)?;
    let conda_list = conda_list(env_name)?;

    if !env_exists(env_name)? {
        return Err(CondaError::EnvNotFound(env_name.to_string(), format!("{:?}", conda_env_list()?)));
    }

    let name = conda_env_export.name;
    let channels = conda_env_export.channels;
    let mut conda_deps: Vec<CondaPackage> = Vec::new();
    let mut pip_deps: Vec<CondaPackage> = Vec::new();
    for package in &conda_list {
        let conda_deps_from_history: Vec<&str> = conda_env_from_history.conda_deps
            .iter()
            .map(|e| e.name.as_str())
            .collect();
        if conda_deps_from_history.contains(&package.name.as_str()) {
            conda_deps.push(package.clone());
        }
        if package.channel.as_deref() == Some("pypi") {
            pip_deps.push(package.clone());
        }
    }

    Ok(CondaEnv { name, channels, conda_deps, pip_deps })
}

#[derive(Debug, Deserialize)]
struct CondaEnvExportYaml {
    name: String,
    channels: Vec<String>,
    dependencies: Vec<serde_yaml::Value>,
}

pub fn conda_env_export(env_name: &str, from_history: bool) -> Result<CondaEnv, CondaError> {
    if !env_exists(env_name)? {
        return Err(CondaError::EnvNotFound(env_name.to_string(), format!("{:?}", conda_env_list()?)));
    }
    
    let args = if from_history {
        vec!["env", "export", "--from-history", "-n", env_name]
    } else {
        vec!["env", "export", "-n", env_name]
    };

    let output = conda_command(args)?;

    let yaml = String::from_utf8_lossy(&output.stdout);
    let parsed: CondaEnvExportYaml = serde_yaml::from_str(&yaml)?;

    let dependencies = parsed.dependencies.iter()
        .filter_map(|dep| dep.as_str().map(|s| {
            let mut parts = s.split("=");
            let name = parts.next().unwrap_or("").to_string();
            let version = parts.next().map(|s| s.to_string());
            let build = parts.next().map(|s| s.to_string());
            let channel = None;
            CondaPackage { name, version, build, channel }
        }))
        .collect();

    Ok(CondaEnv {
        name: parsed.name,
        channels: parsed.channels,
        conda_deps: dependencies,
        pip_deps: Vec::new()
    })
}

pub fn conda_list(env_name: &str) -> Result<Vec<CondaPackage>, CondaError> {
    let output = conda_command(["list", "-n", env_name, "--json"])?;

    let raw: Vec<CondaPackage> = serde_json::from_slice(&output.stdout)?;
    let packages = raw
        .into_iter()
        .map(|e| CondaPackage {
            name: e.name,
            version: e.version,
            build: e.build,
            channel: e.channel,
        })
        .collect();

    Ok(packages)
}

pub fn conda_command<I, S>(args: I) -> Result<Output, CondaError>
where 
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
    std::string::String: From<S>,
{
    let mut command = Command::new("conda");
    let command = command.args(args);
    let output = command.output().map_err(|e| {
        CondaError::CommandExecutionFailed(format!("{e}"))
    })?;

    if !output.status.success() {
        let command_str = command.get_args()
            .map(|s| s.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");
        let err_str = String::from_utf8_lossy(&output.stderr);
        return Err(CondaError::CondaCommandFailed(command_str, err_str.into()));
    }

    Ok(output)
}

pub fn conda_env_list() -> Result<Vec<String>, CondaError> {
    let output = conda_command(["env", "list"])?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let envs: Vec<String> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[0] != "#" {
                Some(parts[0].to_string())
            } else {
                None
            }
        })
        .collect();

    Ok(envs)
}