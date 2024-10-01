use crate::Script;
use url::{Url, Host, Position};
use priority_queue::PriorityQueue;
use std::path::PathBuf;

static DEFAULT_MODULE_DIRECTORY: &str = "modules";

pub trait ModuleLoader{
    /// return the location of the required module
    fn load(&self,request_raiser:&Script,specific:String) -> String;
}

pub struct StandardModuleLoader{
    search_path:PriorityQueue<i64,String>
}

impl StandardModuleLoader{
    pub fn new(home_directory:String) -> StandardModuleLoader{
        let mut paths : PriorityQueue<i64,String> = PriorityQueue::new();
        let home = PathBuf::from(home_directory).canonicalize().unwrap();
        
        paths.push(100,home.join(DEFAULT_MODULE_DIRECTORY).into_os_string().into_string().unwrap());

        return StandardModuleLoader{
            search_path: paths
        }
    }
}

impl ModuleLoader for StandardModuleLoader{
    
    fn load(&self,request_raiser:&Script,specific:String) -> String{
        let search_path = self.search_path.clone();
        search_path.push(request_raiser.path);


    }
}
