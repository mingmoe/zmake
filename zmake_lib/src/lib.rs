pub mod transformer;
pub mod error;
pub mod module;
use serde::{Deserialize, Serialize};

use std::fs;
use std::path::PathBuf;

#[derive(Debug,Clone,Serialize, Deserialize)]
pub enum ScriptType{
    Ecmascript,
    Typescript,
}

/// 代表一个Javascript/Typescript脚本
#[derive(Debug,Clone,Serialize, Deserialize)]
pub struct Script{
    /// 脚本的路径
    pub path:Option<String>,
    pub cached_path:Option<String>,
    /// 脚本内容
    text:Cell<Option<String>>,
    transformed_text:Cell<Option<String>>,
    /// 脚本类型
    pub text_type:ScriptType,
    pub source_map:Option<String>
}

impl std::fmt::Display for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> { 
        write!(f,"script from file:{}; ", 
        self.path.as_ref().unwrap_or(&"unknown file path".to_string()))
    }
}

impl Script{
    pub fn from_ecmascript(text:Option<String>,path:Option<String>) -> Script{
        Script{
            path,
            cached_path:None,
            Cell::new(text),
            transformed_text:None,
            text_type:ScriptType::Ecmascript,
            source_map:None,
        }
    }

    pub fn from_typescript(text:Option<String>,path:Option<String>) -> Script{
        Script{
            path,
            cached_path:None,
            Cell::new(text),
            transformed_text:None,
            text_type:ScriptType::Typescript,
            source_map:None,
        }
    }

    /// get text from cached,or it will read from file
    pub fn get_text(&mut self) -> String{
        if self.text.is_some(){
            Some(self.text.clone().unwrap());
        }
        else{
            let text = fs.read_to_string(self.path.as_ref().unwrap()).unwrap();
            self.text.set(Some(text));
            self.text.clone().unwrap()
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
