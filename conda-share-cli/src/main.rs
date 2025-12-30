use clap::Parser;
use anyhow::Ok;
use std::path::Path;
use std::io::{self, Write};

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

    // If the file exists, ask for confirmation to overwrite
    if output_path.exists() {
        loop {
            print!(
                "Output file '{}' already exists. Overwrite? [y/N]: ",
                output_path.display()
            );
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim().to_lowercase();
            if input == "y" || input == "yes" {
                break;
            } else if input == "n" || input == "no" || input.is_empty() {
                println!("Aborting.");
                return Ok(());
            } else {
                println!("Invalid input. Please enter 'y' or 'n'.");
            }
        }
    }
    
    sharable_conda_env.save(output_path)?;

    Ok(())
}