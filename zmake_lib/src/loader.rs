use crate::options::Options;
use crate::transformer::Transformer;
use crate::{check_script_cache_valid, create_script_cache, Script};
use quickjs_runtime::jsutils::modules::ScriptModuleLoader;
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tracing::trace;

use crate::finder::ModuleFinder;

pub static DEFAULT_CACHED_DIRECTORY: &str = "transformed_cached";

pub struct ModuleLoader {
    pub finder: ModuleFinder,
    pub options: Options,
    pub transformer: Arc<Transformer>,
}

impl ModuleLoader {
    pub fn new(options: Options, transformer: Arc<Transformer>) -> ModuleLoader {
        Self {
            finder: ModuleFinder::new(options.zmake_directory.clone()),
            options,
            transformer,
        }
    }

    pub fn get_cached_file_path(&self, path: &str) -> Option<String> {
        let working_dir = PathBuf::from(&self.options.working_directory);
        let path = PathBuf::from(path);
        let suffix = path.strip_prefix(&working_dir);

        if suffix.is_err() {
            return None;
        }

        let suffix = suffix.unwrap();

        let mut cache_dir = PathBuf::from(&self.options.cache_directory);
        cache_dir.push(&DEFAULT_CACHED_DIRECTORY);
        cache_dir.push(suffix);
        Some(cache_dir.to_str().unwrap().to_string())
    }
}

impl ScriptModuleLoader for ModuleLoader {
    fn normalize_path(
        &self,
        _realm: &QuickJsRealmAdapter,
        ref_path: &str,
        path: &str,
    ) -> Option<String> {
        let mut ref_path = PathBuf::from(ref_path);
        ref_path.pop();

        let found = self.finder.find(
            Some(ref_path.into_os_string().into_string().unwrap()),
            path.to_string(),
        )?;

        // let cached = self.get_cached_file_path(&found);
        Some(found)
    }

    fn load_module(&self, _realm: &QuickJsRealmAdapter, absolute_path: &str) -> String {
        let origin_text = fs::read_to_string(absolute_path).unwrap();

        // for ecmascript,return directly
        if !(absolute_path.ends_with(".ts") || absolute_path.ends_with("mts")) {
            trace!("return ecmascript file: {}", absolute_path);
            return origin_text;
        }

        let cached = self.get_cached_file_path(absolute_path);

        if let Some(cached_file) = cached {
            if fs::exists(&cached_file).unwrap() {
                let cache_text = fs::read_to_string(&cached_file).unwrap();

                if check_script_cache_valid(&origin_text, &cache_text) {
                    trace!("load cache file: {}", cached_file);
                    return cache_text;
                }
            }

            // create updated cache file
            let transformed_text = self
                .transformer
                .transpile(Script::from_typescript(
                    Some(origin_text.clone()),
                    Some(absolute_path.to_string()),
                ))
                .unwrap()
                .get_text();

            trace!("create updated cache file: {}", cached_file);

            let cache_text = create_script_cache(origin_text.clone(), transformed_text);
            let dir = Path::new(&cached_file).parent().unwrap();
            if !fs::exists(dir).unwrap() {
                fs::create_dir_all(&dir).unwrap();
            }
            fs::write(cached_file, cache_text).unwrap();
        }

        // can not be cached, transpile directly
        trace!("transpile typescript file: {}", absolute_path);
        self.transformer
            .transpile(Script::from_typescript(
                Some(origin_text),
                Some(absolute_path.to_string()),
            ))
            .unwrap()
            .get_text()
    }
}
