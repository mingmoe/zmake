use std::fs;

use clap::Parser;
use tracing::{info, trace,error};
use zmake_lib::config::Options;
use zmake_lib::futures::executor::block_on;
use zmake_lib::quickjs_runtime::jsutils::Script as JScript;
use zmake_lib::{engine::Engine, transformer::Transformer, Script};
use zmake_lib::quickjs_runtime::values::JsValueFacade;

#[derive(Parser, Debug)]
#[command(version, about = "a tool to manage source code(download,compile,install and so on)", long_about = None)]
struct Args {
    #[arg(short, long)]
    source_directory: String,

    #[arg(short, long)]
    binary_directory: String,

    #[arg(long)]
    zmake_directory: String,

    #[arg(short, long)]
    cache_directory: String,

    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let source_directory = fs::canonicalize(args.source_directory)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let binary_directory = fs::canonicalize(args.binary_directory)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let zmake_directory = fs::canonicalize(args.zmake_directory)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let cache_directory = fs::canonicalize(args.cache_directory)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let fmt = tracing_subscriber::fmt()
        .with_target(false)
        .with_file(false)
        .with_line_number(false)
        .with_ansi(true)
        .with_level(true);

    if args.debug {
        fmt.pretty()
            .with_line_number(true)
            .with_file(true)
            .with_target(true)
            .with_max_level(tracing::Level::TRACE)
            .init();
    } else {
        fmt.without_time()
            .with_max_level(tracing::Level::INFO)
            .init();
    }

    let options = Options {
        zmake_directory: zmake_directory.clone(),
        source_directory: source_directory.clone(),
        cache_directory: cache_directory.clone(),
        binary_directory: binary_directory.clone(),
        debug: args.debug,
    };

    let engine = Engine::new(options);

    engine
        .runtime
        .set_function(&["console"], "log", |ctx, args| {
            let a = args[0].get_str().to_string();
            info!("{}", a.trim());
            return Ok(zmake_lib::quickjs_runtime::values::JsValueFacade::Null);
        })
        .unwrap();

    let startup_script = JScript::new(
        &format!("{}/./_built_in_fake_file_in_memory_.ts", &source_directory),
        zmake_lib::START_SCRIPT,
    );

    let jsvf = block_on(engine.runtime.eval_module(None, startup_script)).unwrap();
    if let JsValueFacade::JsPromise { cached_promise } = jsvf {
        let result = block_on(cached_promise.get_promise_result()).unwrap().unwrap();
        let result = block_on(result.to_serde_value());

        match result{
            Ok(v)=>trace!("eval_module result:{:?}",v),
            Err(e)=>error!("eval_module result with error:{:?}",e),
        }        
    }
}
