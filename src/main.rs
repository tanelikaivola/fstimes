use chrono::prelude::{DateTime, Utc};
use std::fs;
use walkdir::WalkDir;

static FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f";

mod cli;
use cli::{Cli, Parser};

// 190976
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let path = if args.relative {
        args.path
    } else {
        args.path.canonicalize()?
    };

    for entry in WalkDir::new(path) {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                print_path_metadata(path);
            }
            Err(e) => println!("-\tE\t-\t{e}"),
        }
    }

    Ok(())
}

fn print_path_metadata(path: &std::path::Path) {
    match fs::metadata(path) {
        Ok(metadata) => {
            if let Ok(t) = metadata.modified() {
                let dt: DateTime<Utc> = t.into();
                println!("{}\tM\t{}", dt.format(FORMAT), path.display());
            }
            if let Ok(t) = metadata.accessed() {
                let dt: DateTime<Utc> = t.into();
                println!("{}\tA\t{}", dt.format(FORMAT), path.display());
            }
            if let Ok(t) = metadata.created() {
                let dt: DateTime<Utc> = t.into();
                println!("{}\tC\t{}", dt.format(FORMAT), path.display());
            }
        }
        Err(e) => {
            println!("-\tE\t{}\t{e}", path.display());
        }
    }
}
