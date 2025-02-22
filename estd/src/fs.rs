use std::fs;

use quickjs_runtime::jsutils::modules::{NativeModuleLoader, ScriptModuleLoader};
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use quickjs_runtime::quickjsvalueadapter::QuickJsValueAdapter;
use quickjs_runtime::values::JsValueFacade;
use tracing::debug;

pub struct FsNativeModule {}
pub struct FsScriptModule {}

impl ScriptModuleLoader for FsScriptModule {
    fn normalize_path(
        &self,
        _realm: &QuickJsRealmAdapter,
        _ref_path: &str,
        path: &str,
    ) -> Option<String> {
        if path == "estd.fs" {
            return Some("estd.fs".to_string());
        }
        None
    }

    fn load_module(&self, _realm: &QuickJsRealmAdapter, _absolute_path: &str) -> String {
        include_str!("estd.fs.js").to_string()
    }
}

impl NativeModuleLoader for FsNativeModule {
    fn has_module(&self, _q_ctx: &QuickJsRealmAdapter, module_name: &str) -> bool {
        module_name.eq("estd.internal.fs")
    }

    fn get_module_export_names(
        &self,
        _q_ctx: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<&str> {
        vec!["readFileAsync", "writeFileAsync"]
    }

    fn get_module_exports(
        &self,
        q_ctx: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<(&str, QuickJsValueAdapter)> {
        let mut exports: Vec<(&str, QuickJsValueAdapter)> = Vec::new();

        exports.push((
            "readFileAsync",
            q_ctx
                .create_function_async(
                    "readFileAsync",
                    async |_this, args| {
                        let path = args.get(0).unwrap().get_str();
                        let contents = tokio::fs::read_to_string(path).await.unwrap();
                        debug!("read {}", path);
                        Ok(JsValueFacade::new_string(contents))
                    },
                    1,
                )
                .unwrap(),
        ));

        exports.push((
            "writeFileAsync",
            q_ctx
                .create_function_async(
                    "writeFileAsync",
                    async |_this, args| {
                        let path = args.get(0).unwrap().get_str();
                        let contents = args.get(1).unwrap().get_str();
                        if !fs::exists(path).unwrap() {
                            debug!("create {}", path);
                            fs::File::create(path).unwrap();
                        }
                        debug!("write {}", path);
                        tokio::fs::write(path, contents.as_bytes()).await.unwrap();
                        Ok(JsValueFacade::Null)
                    },
                    2,
                )
                .unwrap(),
        ));

        exports
    }
}
