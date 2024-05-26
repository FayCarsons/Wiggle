use std::collections::VecDeque;

use hashbrown::{HashMap, HashSet};

use crate::{walker::DirWalker, IGNORE_LIST};

type Queue<T> = VecDeque<T>;

pub struct DependencyGraph {
    dependencies: HashMap<String, HashSet<String>>,
    dependents: HashMap<String, usize>,
}

impl DependencyGraph {
    pub fn with_modules(walker: &DirWalker) -> Result<Self, String> {
        let dependencies = Self::get_dependencies(&walker.modules)?;

        #[cfg(debug_assertions)]
        println!("DependencyGraph: {:#?}", dependencies);

        let mut dependents = Self::get_dependents(&dependencies);

        #[cfg(debug_assertions)]
        println!("Dependent graph: {:#?}", dependents);

        dependents.shrink_to_fit();
        Ok(Self {
            dependencies,
            dependents,
        })
    }

    fn get_dependencies(
        modules: &HashMap<String, String>,
    ) -> Result<HashMap<String, HashSet<String>>, String> {
        let mut dependencies = HashMap::with_capacity(modules.len());

        for (module, contents) in modules {
            if IGNORE_LIST.contains(&module.as_bytes()) {
                continue;
            }
            let deps = contents
                .split('\n')
                .enumerate()
                .filter_map(|(idx, s)| {
                    if s.starts_with("#use") {
                        Some(
                            s.split_once(' ')
                                .map(|(_, s)| s.trim().to_lowercase().to_owned())
                                .ok_or(format!(
                                    "{module}::<{idx}> : malformed use statement \"{s}\""
                                )),
                        )
                    } else {
                        None
                    }
                })
                .collect::<Result<HashSet<String>, String>>()?;
            dependencies.insert(module.clone(), deps);
        }

        Ok(dependencies)
    }

    fn get_dependents(dependencies: &HashMap<String, HashSet<String>>) -> HashMap<String, usize> {
        let mut dependents = HashMap::<String, usize>::from_iter(
            dependencies.keys().cloned().zip(std::iter::repeat(0usize)),
        );

        for (_, deps) in dependencies {
            for dep in deps.iter() {
                *dependents.entry(dep.clone()).or_default() += 1;
            }
        }

        dependents
    }

    pub fn topological_sort(mut self) -> Result<Vec<String>, String> {
        let mut queue = self
            .dependents
            .iter()
            .filter_map(|(module, dependents)| {
                if *dependents == 0 {
                    Some(module.clone())
                } else {
                    None
                }
            })
            .collect::<Queue<String>>();

        let mut acc = vec![];

        while let Some(ref node) = queue.pop_front() {
            acc.push(node.clone());

            #[cfg(debug_assertions)]
            println!("DEPENDENTS IN SORT LOOP: {:#?}", self.dependents.clone());

            if let Some(node_deps) = self.dependencies.get(node) {
                for dep in node_deps {
                    if let Some(num_deps) = self.dependents.get_mut(dep) {
                        *num_deps -= 1;
                        if *num_deps == 0 {
                            queue.push_back(dep.clone())
                        }
                    }
                }
            }
        }

        acc.reverse();
        Ok(acc)
    }
}
