use std::sync::Arc;

use crate::get_absolute_path;
use crate::options::Options;
use crate::transformer::Transformer;
use crate::{loader::ModuleLoader, transformer};
use quickjs_runtime::{builder::QuickJsRuntimeBuilder, facades::QuickJsRuntimeFacade};

pub struct Engine {
    pub runtime: QuickJsRuntimeFacade,
    pub transformer: Arc<Transformer>,
}

impl Engine {
    pub fn new(options: Options) -> Engine {
        // check options
        let mut options = options;
        options.zmake_directory = get_absolute_path(&options.zmake_directory);
        options.working_directory = get_absolute_path(&options.working_directory);
        options.cache_directory = get_absolute_path(&options.cache_directory);

        // create runtime
        let transformer = Arc::new(transformer::Transformer::default());

        let runtime = QuickJsRuntimeBuilder::new()
            .memory_limit(1024 * 1024 * 16)
            .max_stack_size(1024 * 1024)
            .script_module_loader(ModuleLoader::new(options.clone(), transformer.clone()))
            .native_module_loader(crate::modules::ConfigurationModule::new(options.clone()))
            .build();

        quickjs_runtime::features::init(&runtime).unwrap();

        return Engine {
            runtime,
            transformer,
        };
    }
}
