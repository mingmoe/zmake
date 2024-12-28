use crate::Script;
use url::{Url, Host, Position};
use priority_queue::PriorityQueue;
use std::path::PathBuf;

static DEFAULT_MODULE_DIRECTORY: &str = "modules";

#[derive(Clone,PartialEq,Eq)]
pub struct ModuleRequest{
    pub specific:String,
    pub module_uri:String,
}

impl ModuleRequest{
    pub fn new(specific:String,module_uri:String) -> ModuleRequest{
        ModuleRequest{
            specific,
            module_uri
        }
    }
}

pub struct ModuleMap{
    target:ModuleRequest,
    dependencies:Vec<ModuleRequest>
}

pub trait ModuleLoader{
    fn resolve(&self,requester:Option<Script>,specific:String) -> Result<ModuleRequest,crate::error::Error>;
    fn load(&self,request:ModuleRequest) -> Result<Script,crate::error::Error>;
}

pub struct StandardModuleLoader{
    search_path:PriorityQueue<String,i64>,
    suffixes:PriorityQueue<String,i64>
}

impl StandardModuleLoader{
    pub fn new(home_directory:String) -> StandardModuleLoader{
        let mut paths : PriorityQueue<String,i64> = PriorityQueue::new();
        let home = PathBuf::from(home_directory).canonicalize().unwrap();
        
        paths.push(home.join(DEFAULT_MODULE_DIRECTORY).into_os_string().into_string().unwrap(),100);

        let mut suffixes : PriorityQueue<String,i64> = PriorityQueue::new();
        suffixes.push( ".ts".to_string(),100);
        suffixes.push( ".mts".to_string(),90);
        suffixes.push( ".js".to_string(),80);
        suffixes.push( ".mts".to_string(),70);

        return StandardModuleLoader{
            search_path: paths,
            suffixes
        }
    }
}

impl ModuleLoader for StandardModuleLoader{
    
    fn resolve(&self,requester:Option<Script>,specific:String) -> Result<ModuleRequest,crate::error::Error>{




    }

    fn load(&self,request:ModuleRequest) -> Result<Script,crate::error::Error>{

    }
}
