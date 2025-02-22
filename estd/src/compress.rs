
use quickjs_runtime::jsutils::modules::NativeModuleLoader;
use quickjs_runtime::jsutils::modules::ScriptModuleLoader;
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use quickjs_runtime::quickjsvalueadapter::QuickJsValueAdapter;
use quickjs_runtime::values::JsValueFacade;
use std::fs::File;
use tar::Archive;
use zip::ZipArchive;

pub struct CompressScriptModule {}
impl ScriptModuleLoader for CompressScriptModule {
    fn normalize_path(
        &self,
        _realm: &QuickJsRealmAdapter,
        _ref_path: &str,
        path: &str,
    ) -> Option<String> {
        if path == "estd.compress" {
            return Some("estd.compress".to_string());
        }
        None
    }

    fn load_module(&self, _realm: &QuickJsRealmAdapter, _absolute_path: &str) -> String {
        include_str!("estd.compress.js").to_string()
    }
}

pub struct CompressNativeModule {}

impl NativeModuleLoader for CompressNativeModule {
    fn has_module(&self, _q_ctx: &QuickJsRealmAdapter, module_name: &str) -> bool {
        module_name.eq("estd.internal.compress")
    }

    fn get_module_export_names(
        &self,
        _q_ctx: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<&str> {
        vec!["unzip", "untar"]
    }

    fn get_module_exports(
        &self,
        q_ctx: &QuickJsRealmAdapter,
        _module_name: &str,
    ) -> Vec<(&str, QuickJsValueAdapter)> {
        let mut exports: Vec<(&str, QuickJsValueAdapter)> = Vec::new();

        exports.push((
            "unzip",
            q_ctx
                .create_function_async(
                    "unzip",
                    async |_this, args| {
                        let path = args.get(0).unwrap().get_str();
                        let output = args.get(1).unwrap().get_str();
                        let file = File::open(path).unwrap();
                        let mut a = ZipArchive::new(file).unwrap();
                        a.extract(output).unwrap();
                        Ok(JsValueFacade::Null)
                    },
                    1,
                )
                .unwrap(),
        ));

        exports.push((
            "untar",
            q_ctx
                .create_function_async(
                    "untar",
                    async |_this, args| {
                        let path = args.get(0).unwrap().get_str();
                        let output = args.get(1).unwrap().get_str();
                        let file = File::open(path).unwrap();
                        let mut a = Archive::new(file);
                        a.unpack(output).unwrap();
                        Ok(JsValueFacade::Null)
                    },
                    2,
                )
                .unwrap(),
        ));

        exports
    }
}
