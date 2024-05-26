#![feature(iter_intersperse)]
mod config;
mod sort;
mod walker;

use std::fs;
use std::{path::PathBuf, str::FromStr};

use crate::config::{Config, EdnConfig};
use crate::sort::DependencyGraph;

const IGNORE_LIST: [&[u8]; 3] = [b"out.bend", b"main.bend", b"build"];

fn main() -> Result<(), String> {
    //#[cfg(debug_assertions)]
    std::env::set_current_dir("/Users/fay/Desktop/Code/bend/pathtracer").unwrap();

    let out = PathBuf::from_str("./out.bend").unwrap();
    if out.exists() {
        fs::remove_file(out).unwrap();
    }

    let config_path = PathBuf::from_str("deps.edn").unwrap();
    assert!(
        config_path.exists(),
        "Cannot find \'deps.edn\' in current directory!"
    );
    assert!(
        PathBuf::from_str("main.bend").unwrap().exists(),
        "Project must have \'main.bend\'"
    );

    let config = fs::read_to_string(config_path).expect("Cannot read \'deps.edn\'");
    let config: Config = serde_json::from_str::<EdnConfig>(&config)
        .expect("Invalid syntax in \'deps.edn\'")
        .parse_sources()?;
    let modules = config.to_modules()?;
    let ordered = DependencyGraph::with_modules(&modules)?.topological_sort()?;

    let output = ordered.iter().try_fold(String::new(), |mut acc, name| {
        if let Some(contents) = modules.modules.get(name) {
            acc.extend(
                contents
                    .lines()
                    .filter(|line| !line.starts_with("#"))
                    .intersperse("\n"),
            );
            Ok(acc)
        } else {
            Err(format!(
                "Error fetching module \'{name}\' from module table"
            ))
        }
    })?;

    fs::write("out.bend", output).map_err(|e| format!("Error writing \'out.bend\': {e}"))?;

    println!("config: {config:#?}\nordered: {ordered:#?}");
    Ok(())
}
