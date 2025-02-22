use crate::ValueWithPriority;
use std::collections::BinaryHeap;
use std::path::PathBuf;
use std::fs;
use tracing::debug;

pub static DEFAULT_MODULE_DIRECTORY: &str = "modules";

#[derive(Debug, Clone)]
pub struct ModuleFinder {
    /// make sure they are absolutely path
    search_path: BinaryHeap<ValueWithPriority<String>>,
    suffixes: BinaryHeap<ValueWithPriority<String>>,
}

impl ModuleFinder {
    pub fn new(home_directory: String) -> ModuleFinder {
        let mut paths: BinaryHeap<ValueWithPriority<String>> = BinaryHeap::new();
        let home = PathBuf::from(home_directory).canonicalize().unwrap();

        paths.push(ValueWithPriority::new(
            home.join(DEFAULT_MODULE_DIRECTORY)
                .into_os_string()
                .into_string()
                .unwrap(),
            0,
        ));

        let mut suffixes: BinaryHeap<ValueWithPriority<String>> = BinaryHeap::new();
        // typescript type
        suffixes.push(ValueWithPriority::new("ts".to_string(), 100));
        suffixes.push(ValueWithPriority::new("mts".to_string(), 90));
        // ecmascript type
        suffixes.push(ValueWithPriority::new("js".to_string(), 80));
        suffixes.push(ValueWithPriority::new("mts".to_string(), 70));

        return ModuleFinder {
            search_path: paths,
            suffixes,
        };
    }

    pub fn find(&self, current_directory: Option<String>, request: String) -> Option<String> {
        debug!(
            "try find module `{}` in current directory: {:?}",
            request, current_directory
        );

        let mut paths = self.search_path.clone();

        if let Some(current) = current_directory {
            if fs::exists(&current).unwrap() {
                paths.push(ValueWithPriority::new(
                    fs::canonicalize(current)
                        .unwrap()
                        .into_os_string()
                        .into_string()
                        .unwrap(),
                    100,
                ));
            } else {
                debug!("ignore inalid current path:{}", current);
            }
        }

        let mut paths = paths.into_sorted_vec();
        paths.reverse();

        let mut suffixes = self.suffixes.clone().into_sorted_vec();
        suffixes.reverse();

        for path in paths {
            debug!("try find module under path:{}", path.value);

            let mut path = PathBuf::from(path.value.clone());
            path.push(request.clone());
            for suffix in suffixes.iter() {
                path.set_extension(suffix.value.clone());
                debug!("try find module:{}", path.to_str().unwrap());
                if path.exists() {
                    path.canonicalize().unwrap();
                    debug!("find module:{}", path.to_str().unwrap());
                    return Some(path.into_os_string().into_string().unwrap());
                }
            }
        }

        return None;
    }
}
