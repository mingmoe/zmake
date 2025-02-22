mod compress;
mod fs;
mod net;

use compress::{CompressNativeModule, CompressScriptModule};
use fs::{FsNativeModule, FsScriptModule};
use quickjs_runtime::builder::QuickJsRuntimeBuilder;
use quickjs_runtime::jsutils::modules::NativeModuleLoader;
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use quickjs_runtime::quickjsvalueadapter::QuickJsValueAdapter;
use std::vec;

pub struct EstdModule {
    pub enabled_fs: bool,
    pub enabled_compress: bool,
}

impl NativeModuleLoader for EstdModule {
    fn has_module(&self, _q_ctx: &QuickJsRealmAdapter, module_name: &str) -> bool {
        module_name.eq("estd")
    }

    fn get_module_export_names(
        &self,
        _q_ctx: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<&str> {
        vec!["hasModule"]
    }

    fn get_module_exports(
        &self,
        q_ctx: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<(&str, QuickJsValueAdapter)> {
        let mut exports: Vec<(&str, QuickJsValueAdapter)> = Vec::new();

        let enabled_fs = self.enabled_fs;
        let enabled_compress = self.enabled_compress;

        // should be a constant value,move it
        exports.push((
            "hasModule",
            q_ctx
                .create_function(
                    "hasModule",
                    move |realm, _this, vec| {
                        let module_name = vec[0].to_string().unwrap();
                        let result = match &*module_name {
                            "fs" => enabled_fs,
                            "compress" => enabled_compress,
                            "console" => true,
                            _ => false,
                        };
                        realm.create_boolean(result)
                    },
                    1,
                )
                .unwrap(),
        ));

        exports
    }
}

pub fn init(
    builder: QuickJsRuntimeBuilder,
    enable_fs: bool,
    enable_compress: bool,
) -> QuickJsRuntimeBuilder {
    let module = EstdModule {
        enabled_fs: enable_fs,
        enabled_compress: enable_compress,
    };

    let mut builder = builder.native_module_loader(module);

    if enable_fs {
        builder = builder
            .native_module_loader(FsNativeModule {})
            .script_module_loader(FsScriptModule {});
    }

    if enable_compress {
        builder = builder
            .native_module_loader(CompressNativeModule {})
            .script_module_loader(CompressScriptModule {});
    }

    builder
}
