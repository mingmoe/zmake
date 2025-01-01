use std::fs;

use clap::Parser;
use tracing::{info, trace};
use zmake_lib::futures::executor::block_on;
use zmake_lib::options::Options;
use zmake_lib::quickjs_runtime::jsutils::Script as JScript;
use zmake_lib::{engine::Engine, transformer::Transformer, Script};

#[derive(Parser, Debug)]
#[command(version, about = "a tool to manage source code(download,compile,install and so on)", long_about = None)]
struct Args {
    #[arg(short, long)]
    working_directory: Option<String>,

    #[arg(long)]
    zmake_directory: String,

    #[arg(short, long)]
    cache_directory: String,

    #[arg(short, long)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    let working_directory = fs::canonicalize(args.working_directory.as_deref().unwrap_or("."))
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
        working_directory: working_directory.clone(),
        cache_directory: cache_directory.clone(),
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

    let my_script = JScript::new(
        &format!("{}/./_built_in_fake_file_in_memory_.ts", &working_directory),
        zmake_lib::START_SCRIPT,
    );

    block_on(engine.runtime.eval_module(None, my_script)).unwrap();
}
