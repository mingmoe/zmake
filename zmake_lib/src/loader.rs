use quickjs_runtime::builder::QuickJsRuntimeBuilder;
use quickjs_runtime::jsutils::modules::ScriptModuleLoader;
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use crate::transformer::Transformer;
use std::fs;
use std::rc::Rc;
use std::path::PathBuf;
use std::sync::Arc;
use crate::Script;

use crate::module::ModuleFinder;

pub static DEFAULT_CACHED_DIRECTORY: &str = "transformed_cached";

pub struct ModuleLoader {
    pub finder:ModuleFinder,
    pub working_dir:String,
    pub cache_dir:String,
    pub transformer:Arc<Transformer>
}

impl ModuleLoader{
    pub fn new(home:String,working_dir:String,cache_dir:String,transformer:Arc<Transformer>) -> ModuleLoader {
        Self {
            finder:ModuleFinder::new(home),
            working_dir,
            cache_dir,
            transformer
        }
    }

    pub fn get_cached_file_path(&self, path: &str) -> Option<String> {
        let mut working_dir = PathBuf::from(&self.working_dir);
        let path = PathBuf::from(path);
        let suffix = path.strip_prefix(&working_dir);

        if suffix.is_err() {
            return None;
        }

        let suffix = suffix.unwrap();

        working_dir.push(&DEFAULT_CACHED_DIRECTORY);
        working_dir.push(suffix);
        working_dir.canonicalize().unwrap();
        Some(working_dir.to_str().unwrap().to_string())
    }
}

impl ScriptModuleLoader for ModuleLoader {
    fn normalize_path(&self, realm: &QuickJsRealmAdapter ,ref_path: &str,path: &str) -> Option<String> {
        let found = self.finder.find(Some(ref_path.to_string()), path.to_string())?;

        // let cached = self.get_cached_file_path(&found);

        // TODO:READ FROM CACHE
        Some(found)
    }

    fn load_module(&self, realm: &QuickJsRealmAdapter, absolute_path: &str) -> String {
        // TODO:JUDGE THE FILE TYPE(TYPESCRIPT OR ECMASCRIPT),WITHOUT HARDCODE

        let text = fs::read_to_string(absolute_path).unwrap();

        if absolute_path.ends_with(".ts") || absolute_path.ends_with("mts"){
            self.transformer.transpile(Script::from_typescript(Some(text), Some(absolute_path.to_string()))).unwrap().get_transformed_text()
        }
        else{
            text
        }
    }
}
