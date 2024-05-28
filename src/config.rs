use hashbrown::HashMap;
use std::{path::PathBuf, str::FromStr};
use url::Url;

#[derive(Clone, Debug)]
pub enum Source {
    Local(PathBuf),
    GitHub(Url),
}

#[derive(Clone, Debug)]
pub struct Config {
    pub deps: HashMap<String, Source>,
}

impl edn_rs::Deserialize for Config {
    fn deserialize(edn: &edn_rs::Edn) -> Result<Self, edn_rs::EdnError> {
        let deps = edn[":deps"]
            .map_iter()
            .ok_or(edn_rs::EdnError::ParseEdn(
                ":deps key not present in \'deps.edn\'".to_string(),
            ))?
            .map(|(dep, source)| {
                let dep = &dep[1..];
                let package_source = if let Some(url) = source.get(":git") {
                    let as_str = url.to_string();
                    Source::GitHub(url::Url::parse(as_str.trim_matches('\"')).map_err(|e| format!("Invalid git url in package {}:\n {as_str:?} -> {e}", dep))?)
                } else if let Some(path) = source.get(":path") {
                    let path = path.to_string();
                   Source::Local(PathBuf::from_str(path.trim_matches(&['\\', '\"'])).map_err(|_| edn_rs::EdnError::ParseEdn(format!("Invalid path for module {dep}: {path}")))?) 
                } else {
                    return Err(edn_rs::EdnError::Deserialize(format!("Package {dep} is missing a \':path\' or \':git\' field in its configuration")))
                };

                Ok((dep.to_string(), package_source))
            })
            .collect::<Result<HashMap<String, Source>, edn_rs::EdnError>>()?;

        Ok(Self { deps })
    }
}
