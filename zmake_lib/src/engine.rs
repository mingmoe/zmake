use std::sync::Arc;

use crate::config::Options;
use crate::get_absolute_path;
use crate::transformer::Transformer;
use crate::{loader::ModuleLoader, transformer};
use estd::init;
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
        options.source_directory = get_absolute_path(&options.source_directory);
        options.cache_directory = get_absolute_path(&options.cache_directory);

        // create runtime
        let transformer = Arc::new(transformer::Transformer::default());

        let builder = QuickJsRuntimeBuilder::new()
            .memory_limit(1024 * 1024 * 16)
            .max_stack_size(1024 * 1024)
            .script_module_loader(ModuleLoader::new(options.clone(), transformer.clone()))
            .native_module_loader(crate::modules::ConfigurationModule::new(options.clone()));

        let builder = init(builder, true);

        let runtime = builder.build();

        quickjs_runtime::features::init(&runtime).unwrap();

        return Engine {
            runtime,
            transformer,
        };
    }
}
