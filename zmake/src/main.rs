use std::fs;

use tracing::{info, trace};
use zmake_lib::futures::executor::block_on;
use zmake_lib::quickjs_runtime::builder::QuickJsRuntimeBuilder;
use zmake_lib::quickjs_runtime::jsutils::Script as JScript;
use zmake_lib::{engine::Engine, transformer::Transformer, Script};

fn main() {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_ansi(true)
        .pretty()
        .with_max_level(tracing::Level::TRACE)
        .with_level(true)
        .init();

    let engine = Engine::new(
        "./home".to_string(),
        "./workdir".to_string(),
        "./cache".to_string(),
    );

    let path = fs::canonicalize("./")
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    /*
    let text = fs::read_to_string(&path).unwrap();

    let script = Script::from_typescript(Some(text), Some(path.clone()));

    let text = engine.transformer.transpile(script).unwrap().get_text();
    */

    engine
        .runtime
        .set_function(&["console"], "log", |ctx, args| {
            let a = args[0].get_str();
            info!("{}", a);
            return Ok(zmake_lib::quickjs_runtime::values::JsValueFacade::Null);
        })
        .unwrap();

    let my_script = JScript::new(&path, 
        "console.log(\"load zmake.ts\")\nimport * as zmake from \"zmake.ts\";");

    block_on(engine.runtime.eval(None, my_script)).unwrap();
}
