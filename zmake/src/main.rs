use std::fs;

use zmake_lib::{transformer::Transformer, Script,engine::Engine};
use zmake_lib::quickjs_runtime::builder::QuickJsRuntimeBuilder;
use zmake_lib::quickjs_runtime::jsutils::Script as JScript;
use zmake_lib::futures::executor::block_on;

fn main(){
    let engine = Engine::new("./home".to_string(), "./workdir".to_string(), "./cache".to_string());

    let path = fs::canonicalize("./zmake.ts").unwrap().into_os_string().into_string().unwrap();

    let text = fs::read_to_string(&path).unwrap();

    let script = Script::from_typescript(Some(text), Some(path.clone()));

    let text = engine.transformer.transpile(script).unwrap().get_transformed_text();

    let my_script = JScript::new(&path, &text);

    engine.runtime.set_function(&["console"], "log", |ctx,args|{
        let a = args[0].get_str();
        println!("{}",a);
        return Ok(zmake_lib::quickjs_runtime::values::JsValueFacade::Null);
    }).unwrap();

    block_on(engine.runtime.eval(None, my_script)).unwrap();
}
