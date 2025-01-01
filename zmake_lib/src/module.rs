use crate::options::Options;
use quickjs_runtime::builder::QuickJsRuntimeBuilder;
use quickjs_runtime::jsutils::modules::NativeModuleLoader;
use quickjs_runtime::jsutils::Script;
use quickjs_runtime::quickjs_utils::functions;
use quickjs_runtime::quickjs_utils::primitives::{from_i32, from_string};
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use quickjs_runtime::quickjsvalueadapter::QuickJsValueAdapter;
use quickjs_runtime::reflection::Proxy;

pub struct NativeModule {
    options: Options,
}

impl NativeModule {
    pub fn new(options: Options) -> NativeModule {
        NativeModule { options }
    }
}

impl NativeModuleLoader for NativeModule {
    fn has_module(&self, _q_ctx: &QuickJsRealmAdapter, module_name: &str) -> bool {
        module_name.eq("zmake")
    }

    fn get_module_export_names(
        &self,
        _q_ctx: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<&str> {
        vec![
            "workingDirectory",
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
            "workingDirectory",
            q_ctx
                .create_string(&self.options.working_directory)
                .unwrap(),
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

        let js_func = functions::new_function_q(
            q_ctx,
            "someFunc",
            |_q_ctx, _this, _args| {
                return Ok(from_i32(432));
            },
            0,
        )
        .ok()
        .unwrap();
        let js_class = Proxy::new()
            .name("SomeClass")
            .static_method("doIt", |_rt, _q_ctx, _args| {
                return Ok(from_i32(185));
            })
            .install(q_ctx, false)
            .ok()
            .unwrap();

        exports
    }
}
