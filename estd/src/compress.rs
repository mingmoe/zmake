use std::fs;

use quickjs_runtime::jsutils::modules::NativeModuleLoader;
use quickjs_runtime::quickjs_utils::atoms::from_string;
use quickjs_runtime::quickjsrealmadapter::QuickJsRealmAdapter;
use quickjs_runtime::quickjsvalueadapter::QuickJsValueAdapter;
use quickjs_runtime::values::JsValueFacade;
use std::io::prelude::*;
use std::fs::File;
use tar::Archive;

pub struct CompressScriptModule
impl ScriptModuleLoader for CompressScriptModule{
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

        exports.push(("unzip",q_ctx.create_function_async("unzip", async |this,args|{
            let path = args.get(0).unwrap().get_str();
            let contents = tokio::fs::read_to_string(path).await.unwrap();
            Ok(JsValueFacade::new_string(contents))
        }, 1).unwrap()));
        
        exports.push(("untar",q_ctx.create_function_async("untar", async |this,args|{
            let path = args.get(0).unwrap().get_str();
            let output = args.get(1).unwrap().get_str();
            let file = File::open(path).unwrap();
            let mut a = Archive::new(file);
            
            for file in a.entries().unwrap() {
                // Make sure there wasn't an I/O error
                let mut file = file.unwrap();
        
                // Inspect metadata about the file
                println!("{:?}", file.header().path().unwrap());
                println!("{}", file.header().size().unwrap());
        
                // files implement the Read trait
                let mut s = String::new();
                file.read_to_string(&mut s).unwrap();
                println!("{}", s);
            }

            Ok(JsValueFacade::Null)
        }, 2).unwrap()));

        exports
    }
}
