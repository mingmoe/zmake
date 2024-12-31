pub mod transformer;    
pub mod error;
pub mod engine;
pub mod module;
pub mod loader;
use highway::{HighwayHasher, HighwayHash};
use core::panic;
use std::fs;
use std::cell::OnceCell;
use std::cmp::{Ord,PartialEq,PartialOrd,Ordering};
pub use quickjs_runtime;
pub use futures;

#[derive(Debug,Clone,PartialEq)]
pub enum ScriptType{
    Ecmascript,
    Typescript,
}

/// 代表一个Javascript/Typescript脚本
#[derive(Debug,Clone)]
pub struct Script{
    /// 脚本的路径
    pub path:Option<String>,
    pub cached_path:Option<String>,
    /// 脚本内容
    text:OnceCell<String>,
    transformed_text:OnceCell<String>,
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
        let cell = if text.is_some(){
            OnceCell::from(text.unwrap())
        }
        else{
            OnceCell::new()
        };

        Script{
            path,
            cached_path:None,
            text:cell,
            transformed_text:OnceCell::new(),
            text_type:ScriptType::Ecmascript,
            source_map:None,
        }
    }

    pub fn from_typescript(text:Option<String>,path:Option<String>) -> Script{
        let cell = if text.is_some(){
            OnceCell::from(text.unwrap())
        }
        else{
            OnceCell::new()
        };

        Script{
            path,
            cached_path:None,
            text:cell,
            transformed_text:OnceCell::new(),
            text_type:ScriptType::Typescript,
            source_map:None,
        }
    }

    /// get text from cached,or it will read from file
    pub fn get_text(&self) -> String{
        self.text.get_or_init(||{
            let path = self.path.as_ref().unwrap();
            fs::read_to_string(path).unwrap()
        }).clone()
    }

    pub fn get_transformed_text(&self) -> String{
        if self.text_type == ScriptType::Ecmascript {
            return self.get_text();
        }

        return self.transformed_text.get_or_init(||{
            if self.cached_path.is_some(){
                let path = self.cached_path.as_ref().unwrap();
                return fs::read_to_string(path).unwrap()
            }
            panic!("transformed text is not cached and cached_path is not set");
        }).clone()
    }

    pub fn to_transformed(&self,transformed_text:String,source_map:Option<String>) -> Script{
        let mut transformed = self.clone();
        transformed.transformed_text.set(transformed_text).unwrap();
        transformed.source_map = source_map;
        transformed
    }

    pub fn save_cached(&self)->Result<(),std::io::Error>{
        if self.text_type == ScriptType::Ecmascript {
            return Ok(());
        }

        let path = self.cached_path.as_ref().unwrap();

        let origin = self.get_text();

        let hash = hash256_to_string(HighwayHasher::default().hash256(origin.as_bytes()));

        let text = format!("//origin file hash:{}\n{}",hash,self.get_transformed_text());
        
        fs::write(path, text)?;
        Ok(())
    }

    /// load transformed text from cached path,
    /// or the cached is invalid, return Ok(None()).
    pub fn load_cached(&self)->Result<Option<String>,std::io::Error>{
        if self.text_type == ScriptType::Ecmascript {
            return Ok(Some(self.get_text()));
        }

        let origin = self.get_text();
        let hash = hash256_to_string(HighwayHasher::default().hash256(origin.as_bytes()));

        let to_match = format!("//origin file hash:{}",hash);

        let path = self.cached_path.as_ref().unwrap();
        let text = fs::read_to_string(path)?;
        if text.starts_with(&to_match) {
            Ok(Some(text))
        } else {
            Ok(None)
        }
    }

}

#[derive(Clone,Debug)]
struct ValueWithPriority<T>{
    pub value:T,
    pub priority:i64
}

impl<T> ValueWithPriority<T>{
    pub fn new(value:T,priority:i64) -> ValueWithPriority<T>{
        ValueWithPriority{
            value,
            priority
        }
    }
}

impl<T> PartialEq for ValueWithPriority<T>{
    fn eq(&self,other:&ValueWithPriority<T>) -> bool{
        self.priority == other.priority
    }
}

impl<T> Eq for ValueWithPriority<T>{
    fn assert_receiver_is_total_eq(&self) {
        self.priority.assert_receiver_is_total_eq();
    }
}

impl<T> PartialOrd for ValueWithPriority<T>{
    fn partial_cmp(&self,other:&ValueWithPriority<T>) -> Option<Ordering>{
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> Ord for ValueWithPriority<T>{
    fn cmp(&self,other:&ValueWithPriority<T>) -> Ordering{
        self.priority.cmp(&other.priority)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_with_priority_ordering_works() {
        let a = ValueWithPriority::new("2",1);
        let b = ValueWithPriority::new("1",2);
        assert_eq!(a.cmp(&b),Ordering::Less);
        assert_eq!(b.cmp(&a),Ordering::Greater);
    }

    #[test]
    fn value_with_priority_equal_works() {
        let a = ValueWithPriority::new("2",1);
        let b = ValueWithPriority::new("1",1);
        assert_eq!(a.cmp(&b),Ordering::Equal);
        assert_eq!(b.cmp(&a),Ordering::Equal);
    }

}

fn hash256_to_string(x:[u64;4])->String{
    let target:[u8;32] = bytemuck::cast(x);

    return target.iter()
        .map(|byte| format!("{:02X}", byte))
        .collect();
}