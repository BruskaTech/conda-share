use clap::Parser;
use anyhow::Ok;
use std::path::Path;

use conda_share_core::*;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Conda environment name
    env_name: String,

    /// The path of output file
    #[arg(short, long)]
    path: Option<String>,

    /// Display the output to stdout instead of saving to a file
    #[arg(short, long, default_value_t = false)]
    display: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Generate the conda sharable environment
    let sharable_conda_env = sharable_env(&args.env_name)?;

    if args.display {
        print!("{}", sharable_conda_env.to_yaml()?);
        return Ok(());
    }

    let file_path = args.path.unwrap_or(args.env_name + ".yml");
    let output_path = Path::new(&file_path);
    sharable_conda_env.save(output_path)?;

    Ok(())
}