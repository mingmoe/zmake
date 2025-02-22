use std::fs;

use quickjs_runtime::jsutils::modules::NativeModuleLoader;
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use quickjs_runtime::quickjsvalueadapter::QuickJsValueAdapter;
use quickjs_runtime::values::JsValueFacade;

pub struct NetModule {}

impl NativeModuleLoader for NetModule {
    fn has_module(&self, _q_ctx: &QuickJsRealmAdapter, module_name: &str) -> bool {
        module_name.eq("estd:net")
    }

    fn get_module_export_names(
        &self,
        _q_ctx: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<&str> {
        vec!["fetch"]
    }

    fn get_module_exports(
        &self,
        q_ctx: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<(&str, QuickJsValueAdapter)> {
        let mut exports: Vec<(&str, QuickJsValueAdapter)> = Vec::new();

        exports.push((
            "readFile",
            q_ctx
                .create_function_async(
                    "readFileAsync",
                    async |_this, args| {
                        let path = args.get(0).unwrap().get_str();
                        let contents = tokio::fs::read_to_string(path).await.unwrap();
                        Ok(JsValueFacade::new_string(contents))
                    },
                    1,
                )
                .unwrap(),
        ));

        exports.push((
            "writeFile",
            q_ctx
                .create_function_async(
                    "readFileAsync",
                    async |_this, args| {
                        let path = args.get(0).unwrap().get_str();
                        let contents = args.get(1).unwrap().get_str();
                        if !fs::exists(path).unwrap() {
                            fs::File::create(path).unwrap();
                        }
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
