
use thiserror::Error;

use crate::Script;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Get a error when transform(at file {script}):{reason}")]
    TransformeError{ 
        script: Script,
        reason:String 
    },
    #[error("Get a error when eval(at file {script}):{reason}")]
    EvalError{ 
        script: Script,
        reason:String 
    },
    #[error("Get a error when resolve the module request(specific:{specific}):{reason}")]
    ModuleRequestError{ 
        specific: Script,
        reason:String 
    },
    #[error("Get a error when load the module request(specific:{specific}):{reason}")]
    ModuleLoadError{ 
        specific: Script,
        reason:String 
    },
}