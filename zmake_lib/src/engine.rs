use std::fs;
use std::sync::Arc;
use std::{rc::Rc, u64};

use crate::transformer::Transformer;
use crate::{loader::ModuleLoader, transformer};
use quickjs_runtime::{builder::QuickJsRuntimeBuilder, facades::QuickJsRuntimeFacade};

pub struct Engine {
    pub runtime: QuickJsRuntimeFacade,
    pub transformer: Arc<Transformer>,
}

impl Engine {
    pub fn new(home: String, working_dir: String, cache_dir: String) -> Engine {
        let transformer = Arc::new(transformer::Transformer::default());

        let home = fs::canonicalize(home)
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        let working_dir = fs::canonicalize(working_dir)
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();
        let cache_dir = fs::canonicalize(cache_dir)
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();

        let runtime = QuickJsRuntimeBuilder::new()
            .memory_limit(1024 * 1024 * 16)
            .max_stack_size(1024 * 1024)
            .script_module_loader(ModuleLoader::new(
                home,
                working_dir,
                cache_dir,
                transformer.clone(),
            ))
            .build();

        return Engine {
            runtime,
            transformer,
        };
    }
}
