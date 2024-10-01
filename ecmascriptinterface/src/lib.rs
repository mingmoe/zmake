pub mod transformer;
pub mod error;
pub mod v8;
pub mod module;
use serde::{Deserialize, Serialize};

use std::fs;
use std::path::PathBuf;

/// 代表一个Javascript/Typescript脚本
#[derive(Debug,Clone)]
pub struct Script{
    /// 脚本的绝对路径
    pub path:Option<String>,
    /// 脚本内容
    pub text:String,
    /// 转译
    pub transformed:Option<String>,
    pub cache:Option<TransformedScriptCache>,
    pub source_map:Option<String>
}

#[derive(Serialize, Deserialize, Debug,Clone)]
pub struct TransformedScriptCache{
    pub origin_path:String,
    pub origin_hash:String,
    pub transformed_path:String,
    pub transformed_text:String
}


impl std::fmt::Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f,"script from file:{}; ", 
        self.path.as_ref().unwrap_or(&"unknown file path".to_string()))
    }
}

impl Script{
    pub fn from(text:String,path:String) -> Script{
        Script{
            text,
            path:Some(path),
            transformed: None,
            cache:None,
            source_map:None
        }
    }

    pub fn from_file(path:&str) -> Script{
        let path:String = PathBuf::from(path).canonicalize().unwrap().into_os_string().into_string().unwrap();

        let text = fs::read_to_string(&path).unwrap();

        Script::from(text,path)
    }

    pub fn with_transformed(self,transformed_text:String,source_map:Option<String>) -> Script{
        Script{
            text:self.text,
            path:self.path,
            transformed:Some(transformed_text),
            cache:None,
            source_map:source_map
        }
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
