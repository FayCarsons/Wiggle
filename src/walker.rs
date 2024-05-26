use hashbrown::HashMap;
use std::{fs, os::unix::prelude::OsStrExt, path::Path};

use crate::IGNORE_LIST;

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
