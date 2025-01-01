pub mod engine;
pub mod error;
pub mod finder;
pub mod loader;
pub mod modules;
pub mod options;
pub mod transformer;
pub use futures;
use highway::{HighwayHash, HighwayHasher};
pub use quickjs_runtime;
use std::cell::OnceCell;
use std::cmp::{Ord, Ordering, PartialEq, PartialOrd};
use std::fs;

#[derive(Debug, Clone, PartialEq)]
pub enum ScriptType {
    Ecmascript,
    Typescript,
}

pub static START_SCRIPT: &str = include_str!("built-in.js");

/// 代表一个Javascript/Typescript脚本
#[derive(Debug, Clone)]
pub struct Script {
    pub path: Option<String>,
    text: OnceCell<String>,
    pub text_type: ScriptType,
    pub source_map: Option<String>,
}

impl std::fmt::Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "script from file:{}; ",
            self.path
                .as_ref()
                .unwrap_or(&"unknown file path".to_string())
        )
    }
}

impl Script {
    pub fn from_ecmascript(text: Option<String>, path: Option<String>) -> Script {
        let cell = if text.is_some() {
            OnceCell::from(text.unwrap())
        } else {
            OnceCell::new()
        };

        Script {
            path,
            text: cell,
            text_type: ScriptType::Ecmascript,
            source_map: None,
        }
    }

    pub fn from_typescript(text: Option<String>, path: Option<String>) -> Script {
        let cell = if text.is_some() {
            OnceCell::from(text.unwrap())
        } else {
            OnceCell::new()
        };

        Script {
            path,
            text: cell,
            text_type: ScriptType::Typescript,
            source_map: None,
        }
    }

    /// get text from cached,or it will read from file
    pub fn get_text(&self) -> String {
        self.text
            .get_or_init(|| {
                let path = self.path.as_ref().unwrap();
                fs::read_to_string(path).unwrap()
            })
            .clone()
    }

    pub fn to_transformed(&self, transformed_text: String, source_map: Option<String>) -> Script {
        let mut script = Script::from_ecmascript(Some(transformed_text), self.path.clone());
        script.source_map = source_map;
        script
    }
}

pub fn create_script_cache(origin_text: String, transformed_text: String) -> String {
    let hash = hash256_to_string(HighwayHasher::default().hash256(origin_text.as_bytes()));

    format!("//origin file hash:{}\n{}", hash, transformed_text)
}

pub fn check_script_cache_valid(origin_text: &str, transformed_text_cache: &str) -> bool {
    let hash = hash256_to_string(HighwayHasher::default().hash256(origin_text.as_bytes()));

    let to_match = format!("//origin file hash:{}", hash);

    transformed_text_cache.starts_with(&to_match)
}

#[derive(Clone, Debug)]
struct ValueWithPriority<T> {
    pub value: T,
    pub priority: i64,
}

impl<T> ValueWithPriority<T> {
    pub fn new(value: T, priority: i64) -> ValueWithPriority<T> {
        ValueWithPriority { value, priority }
    }
}

impl<T> PartialEq for ValueWithPriority<T> {
    fn eq(&self, other: &ValueWithPriority<T>) -> bool {
        self.priority == other.priority
    }
}

impl<T> Eq for ValueWithPriority<T> {
    fn assert_receiver_is_total_eq(&self) {
        self.priority.assert_receiver_is_total_eq();
    }
}

impl<T> PartialOrd for ValueWithPriority<T> {
    fn partial_cmp(&self, other: &ValueWithPriority<T>) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> Ord for ValueWithPriority<T> {
    fn cmp(&self, other: &ValueWithPriority<T>) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_with_priority_ordering_works() {
        let a = ValueWithPriority::new("2", 1);
        let b = ValueWithPriority::new("1", 2);
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(b.cmp(&a), Ordering::Greater);
    }

    #[test]
    fn value_with_priority_equal_works() {
        let a = ValueWithPriority::new("2", 1);
        let b = ValueWithPriority::new("1", 1);
        assert_eq!(a.cmp(&b), Ordering::Equal);
        assert_eq!(b.cmp(&a), Ordering::Equal);
    }
}

fn hash256_to_string(x: [u64; 4]) -> String {
    let target: [u8; 32] = bytemuck::cast(x);

    return target.iter().map(|byte| format!("{:02X}", byte)).collect();
}

fn get_absolute_path(path: &str) -> String {
    let absolute_path = std::fs::canonicalize(path).unwrap();
    return absolute_path.to_str().unwrap().to_string();
}
