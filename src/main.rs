use std::path::Path;
use std::process::Command;
use serde::Deserialize;
use std::fs::File;
use std::io::Write;

fn main() -> anyhow::Result<()> {
    let env = "test_env";
    let output_path = Path::new("env.yml");

    let (_, _, conda_export_from_history) = conda_env_export_from_history(env, true)?;
    let (name, channels, _) = conda_env_export_from_history(env, false)?;

    let conda_list = conda_list(env)?;

    let mut conda_deps: Vec<String> = Vec::new();
    let mut pip_deps: Vec<String> = Vec::new();
    for entry in &conda_list {
        let deps_from_history: Vec<&str> = conda_export_from_history.iter().map(|e| e.name.as_str()).collect();
        if deps_from_history.contains(&entry.name.as_str()) {
            conda_deps.push(format!("{}={}", entry.name, entry.version));
            // println!("Found matching package: {} {} {} {}", entry.name, entry.version, entry.build, entry.channel);
        }
        if entry.channel == "pypi" {
            pip_deps.push(format!("{}=={}", entry.name, entry.version));
        }
    }
    println!("Env Name: {}", name);
    println!("Channels: {:?}", channels);
    println!("Conda Deps: {:?}", conda_deps);
    println!("Pip Deps: {:?}", pip_deps);

    let mut yml = String::new();
    yml.push_str(&format!("name: {}\n", name));

    yml.push_str("channels:\n");
    yml.extend(channels.iter().map(|c| format!("  - {}\n", c)));

    if !conda_deps.is_empty() {
        yml.push_str("dependencies:\n");
        yml.extend(conda_deps.iter().map(|dep| format!("  - {}\n", dep)));
    }

    if !pip_deps.is_empty() {
        yml.push_str("  - pip:\n");
        yml.extend(pip_deps.iter().map(|dep| format!("    - {}\n", dep)));
    }

    let mut file = File::create(output_path)?;
    file.write_all(yml.as_bytes())?;
    println!("Generated env.yml");

    Ok(())
}

fn conda_env_export_from_history(env_name: &str, from_history: bool) -> anyhow::Result<(String, Vec<String>, Vec<CondaListEntry>)> {
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
            CondaListEntry {
                name: s.split('=').next().unwrap_or("").to_string(),
                version: "".to_string(),
                build: "".to_string(),
                channel: "".to_string(),
            }
        }))
        .collect();

    Ok((parsed.name, parsed.channels, dependencies))
}

#[derive(Debug)]
struct CondaListEntry {
    name: String,
    version: String,
    build: String,
    channel: String,
}

fn conda_list(env_name: &str) -> anyhow::Result<Vec<CondaListEntry>> {
    let output = Command::new("conda")
        .args(["list", "-n", env_name, "--json"])
        .output()?;

    if !output.status.success() {
        anyhow::bail!(
            "conda list failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[derive(Deserialize)]
    struct RawEntry {
        name: String,
        version: String,
        build_string: String,
        channel: Option<String>,
    }

    let raw: Vec<RawEntry> = serde_json::from_slice(&output.stdout)?;
    let entries = raw
        .into_iter()
        .map(|e| CondaListEntry {
            name: e.name,
            version: e.version,
            build: e.build_string,
            channel: e.channel.unwrap_or_default(),
        })
        .collect();

    Ok(entries)
}

// fn conda_list(prefix: &Path) -> anyhow::Result<Vec<PrefixRecord>> {
//     let meta_dir = prefix.join("conda-meta");

//     let mut records = Vec::new();
//     for entry in fs::read_dir(meta_dir)? {
//         let entry = entry?;
//         if entry.file_name() == "history" { continue; }
//         let path = entry.path();
//         println!("Found file: {:?}", path);


//         let record = PrefixRecord::from_path(path)?;
//         println!("{}", record.name().as_normalized());
//         // return Ok(vec![record]);

//         // if path.extension().and_then(|s| s.to_str()) == Some("json") {
//         //     let data = fs::read(&path)?;
//         //     let record: PrefixRecord = serde_json::from_slice(&data)?;
//         //     records.push(record);
//         // }
//     }

//     Ok(records)
// }