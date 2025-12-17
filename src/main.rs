use std::path::Path;
use std::process::Command;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

fn main() -> anyhow::Result<()> {
    let env_name = "test_env";
    let output_path = Path::new("env.yml");

    let conda_env = good_export_env(env_name)?;

    conda_env.save(output_path)?;
    println!("Generated {}", output_path.display());

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct CondaEnv {
    name: String,
    channels: Vec<String>,
    conda_deps: Vec<CondaPackage>,
    pip_deps: Vec<CondaPackage>,
}

impl CondaEnv {
    fn to_yaml(&self) -> anyhow::Result<String> {
        let mut yml = String::new();
        yml.push_str(&format!("name: {}\n", self.name));

        yml.push_str("channels:\n");
        yml.extend(self.channels.iter().map(|c| format!("  - {}\n", c)));
        if !self.conda_deps.is_empty() {
            yml.push_str("dependencies:\n");
            for dep in &self.conda_deps {
                let version = dep.version.clone().ok_or(anyhow::anyhow!("Missing version"))?;
                yml.push_str(&format!("  - {}={}\n", dep.name, version));
            }
        }

        if !self.pip_deps.is_empty() {
            yml.push_str("  - pip:\n");
            for dep in &self.pip_deps {
                let version = dep.version.clone().ok_or(anyhow::anyhow!("Missing version"))?;
                yml.push_str(&format!("      - {}=={}\n", dep.name, version));
            }
        }

        Ok(yml)
    }
    fn save(&self, path: &Path) -> anyhow::Result<()> {
        let yml = self.to_yaml()?;
        let mut file = File::create(path)?;
        file.write_all(yml.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct CondaPackage {
    name: String,
    version: Option<String>,
    build: Option<String>,
    channel: Option<String>,
}

fn good_export_env(env_name: &str) -> anyhow::Result<CondaEnv> {
    let conda_env_from_history = conda_env_export(env_name, true)?;
    let conda_env_export = conda_env_export(env_name, false)?;
    let conda_list = conda_list(env_name)?;

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

fn conda_env_export(env_name: &str, from_history: bool) -> anyhow::Result<CondaEnv> {
    let args = if from_history {
        vec!["env", "export", "--from-history", "-n", env_name]
    } else {
        vec!["env", "export", "-n", env_name]
    };

    let output = Command::new("conda").args(&args).output()?;

    if !output.status.success() {
        anyhow::bail!(
            "conda env export failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[derive(Debug, Deserialize)]
    struct CondaEnvExport {
        name: String,
        channels: Vec<String>,
        dependencies: Vec<serde_yaml::Value>,
    }

    let yaml = String::from_utf8_lossy(&output.stdout);
    let parsed: CondaEnvExport = serde_yaml::from_str(&yaml)?;

    // dependencies can be strings or maps (for pip), so filter only string ones for conda deps
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

fn conda_list(env_name: &str) -> anyhow::Result<Vec<CondaPackage>> {
    let output = Command::new("conda")
        .args(["list", "-n", env_name, "--json"])
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "conda list failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

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