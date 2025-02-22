use crate::config::Options;
use quickjs_runtime::jsutils::modules::NativeModuleLoader;
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use quickjs_runtime::quickjsvalueadapter::QuickJsValueAdapter;

/// implements the `zmake:configuration`` module
pub struct ConfigurationModule {
    options: Options,
}

impl ConfigurationModule {
    pub fn new(options: Options) -> ConfigurationModule {
        ConfigurationModule { options }
    }
}

impl NativeModuleLoader for ConfigurationModule {
    fn has_module(&self, _q_ctx: &QuickJsRealmAdapter, module_name: &str) -> bool {
        module_name.eq("zmake.configuration")
    }

    fn get_module_export_names(
        &self,
        _q_ctx: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<&str> {
        vec![
            "sourceDirectory",
            "binaryDirectory",
            "cacheDirectory",
            "zmakeDirectory",
            "debug",
        ]
    }

    fn get_module_exports(
        &self,
        q_ctx: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<(&str, QuickJsValueAdapter)> {
        let mut exports: Vec<(&str, QuickJsValueAdapter)> = Vec::new();

        exports.push((
            "sourceDirectory",
            q_ctx.create_string(&self.options.source_directory).unwrap(),
        ));
        exports.push((
            "binaryDirectory",
            q_ctx.create_string(&self.options.binary_directory).unwrap(),
        ));
        exports.push((
            "cacheDirectory",
            q_ctx.create_string(&self.options.cache_directory).unwrap(),
        ));
        exports.push((
            "zmakeDirectory",
            q_ctx.create_string(&self.options.zmake_directory).unwrap(),
        ));
        exports.push(("debug", q_ctx.create_boolean(self.options.debug).unwrap()));

        exports
    }
}
