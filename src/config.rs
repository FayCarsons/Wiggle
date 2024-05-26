use git2::Repository;
use hashbrown::HashMap;
use serde::Deserialize;
use std::{path::PathBuf, str::FromStr};
use url::Url;

use crate::walker::DirWalker;

#[derive(Clone, Debug)]
pub enum Source {
    Local(PathBuf),
    GitHub(Url),
}

#[derive(Deserialize, Debug)]
pub struct EdnConfig {
    deps: HashMap<String, String>,
}

impl EdnConfig {
    pub fn parse_sources(self) -> Result<Config, String> {
        let mut deps = HashMap::new();
        for (dependency, source) in self.deps {
            if let Some(url) = source.strip_prefix("git+") {
                let url = Url::parse(url).map_err(|e| format!("Invalid GtiHub URL: {e}"))?;
                deps.insert(dependency, Source::GitHub(url));
            } else {
                deps.insert(
                    dependency,
                    Source::Local(
                        PathBuf::from_str(&source).map_err(|e| format!("Invalid path: {e}"))?,
                    ),
                );
            }
        }

        Ok(Config { deps })
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    deps: HashMap<String, Source>,
}

impl Config {
    pub fn to_modules(&self) -> Result<DirWalker, String> {
        let build_dir =
            PathBuf::from_str("./.build/").map_err(|e| format!("Internal error: {e}"))?;

        let mut walker =
            DirWalker::walk_dir(".").map_err(|e| format!("Error collecting local modules: {e}"))?;
        #[cfg(debug_assertions)]
        println!("Local modules: {:#?}", walker.modules.clone().keys());

        for (ref name, source) in self.deps.iter() {
            match source {
                Source::GitHub(url) => {
                    let location = &build_dir.join(name);
                    Repository::clone_recurse(url.as_str(), location)
                        .map_err(|e| format!("Error fetching repo \"{name}\": {e}"))?;
                    walker
                        .fold(location)
                        .map_err(|e| format!("Error collecting modules from repo {url:?}: {e}"))?;
                    #[cfg(debug_assertions)]
                    println!(
                        "Got Git directory {url:?}, modules: {:#?}",
                        walker.modules.keys().clone()
                    )
                }
                Source::Local(ref path) => {
                    walker
                        .fold(path)
                        .map_err(|e| format!("Error collecting modules in {path:?}: {e}"))?;
                    #[cfg(debug_assertions)]
                    println!(
                        "Got local dependency {path:?}, modules: {:#?}",
                        walker.modules.keys().clone()
                    )
                }
            }
        }

        Ok(walker)
    }
}
