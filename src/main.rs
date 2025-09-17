use std::path::PathBuf;
use walkdir::WalkDir;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Find {
        /// lists test values
        #[arg(name = "dirGlob")]
        dir_glob: String,
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    match &cli.command {
        Some(Commands::Find { dir_glob }) => {
            println!("=== Using walkdir to show all paths being searched ===\n");
            
            let mut all_paths = Vec::new();
            let mut matched_paths = Vec::new();
            
            for entry in WalkDir::new(dir_glob)
                .into_iter()
                .filter_entry(|e| {
                    let path = e.path();
                    let is_node_modules = path
                        .file_name()
                        .and_then(|s| s.to_str())
                        .map(|s| s == "node_modules")
                        .unwrap_or(false);
                    
                    if is_node_modules {
                        println!("⛔ Skipping directory: {}", path.display());
                        false
                    } else {
                        if e.file_type().is_dir() {
                            println!("📁 Entering directory: {}", path.display());
                        } else {
                            println!("🔍 Checking file: {}", path.display());
                        }
                        true
                    }
                })
            {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        all_paths.push(path.to_path_buf());
                        
                        if entry.file_type().is_file() {
                            let file_name = path.file_name()
                                .and_then(|s| s.to_str())
                                .unwrap_or("");
                            
                            if file_name.starts_with(".env") {
                                matched_paths.push(path.to_path_buf());
                                println!("  ✅ MATCH: {}", path.display());
                            }
                        }
                    }
                    Err(e) => {
                        println!("  ❌ Error accessing path: {}", e);
                    }
                }
            }
            
            println!("\n=== Summary ===");
            println!("Total paths searched: {}", all_paths.len());
            println!("Matching .env files found: {}", matched_paths.len());
            
            if !matched_paths.is_empty() {
                println!("\nMatched files:");
                for path in &matched_paths {
                    println!("  - {}", path.display());
                }
            }
        }
        None => {}
    }
}
