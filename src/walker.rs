use hashbrown::HashMap;
use std::{
    fs,
    os::unix::prelude::OsStrExt,
    path::{Path, PathBuf},
    str::FromStr,
};

use git2::Repository;

use crate::{
    config::{Config, Source},
    BUILD_DIR, IGNORE_LIST,
};

#[derive(Clone)]
pub struct DirWalker {
    pub size: usize,
    pub modules: HashMap<String, String>,
}

impl DirWalker {
    pub fn new() -> Self {
        Self {
            size: 0,
            modules: HashMap::new(),
        }
    }

    fn is_bend(file: &Path) -> bool {
        file.extension()
            .is_some_and(|ext| ext.as_bytes() == b"bend")
    }

    fn walk<P: AsRef<Path>>(&mut self, root: P) -> std::io::Result<()> {
        for path in root.as_ref().read_dir()?.flatten() {
            if path
                .path()
                .file_stem()
                .is_some_and(|stem| IGNORE_LIST.contains(&stem.as_bytes()))
            {
                continue;
            }
            let file_type = path.file_type()?;
            let path = path.path();
            if file_type.is_dir() {
                self.walk(path)?;
            } else if file_type.is_file() && Self::is_bend(&path) {
                let name = path
                    .file_stem()
                    .and_then(|os| os.to_str())
                    .ok_or(std::io::Error::other(format!(
                        "Error processing file: {path:?}"
                    )))?
                    .trim()
                    .to_lowercase();
                let contents = fs::read_to_string(&path)?;
                self.modules.insert(name.to_owned(), contents);
                self.size += 1;
            }
        }

        Ok(())
    }

    pub fn walk_dir<P: AsRef<Path>>(root: P) -> std::io::Result<Self> {
        let mut walker = Self::new();
        walker.walk(root).map(|_| walker)
    }

    pub fn fold<P: AsRef<Path>>(&mut self, root: P) -> std::io::Result<()> {
        self.walk(root)
    }
}

impl TryFrom<&Config> for DirWalker {
    type Error = String;

    fn try_from(value: &Config) -> Result<Self, Self::Error> {
        let build_dir = PathBuf::from_str(BUILD_DIR).unwrap();

        let mut walker =
            DirWalker::walk_dir(".").map_err(|e| format!("Error collecting local modules: {e}"))?;

        #[cfg(debug_assertions)]
        println!("Local modules: {:#?}", walker.modules.clone().keys());

        for (ref name, source) in value.deps.iter() {
            match source {
                Source::GitHub(url) => {
                    let location = &build_dir.join(name);
                    if location.exists()
                        && location
                            .read_dir()
                            .is_ok_and(|mut dir| dir.next().is_none())
                    {
                        continue;
                    }
                    Repository::clone(url.as_str(), location)
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
                    #[cfg(debug_assertions)]
                    println!("Fetching local dependency: {:#?}", path);

                    walker
                        .fold(path)
                        .map_err(|e| format!("Error collecting modules in {:#?}: {e}", path))?;

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
