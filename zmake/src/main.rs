use zmake_lib::{transformer::Transformer, v8::V8Engine, Script};

fn main(){
    let transformer = Transformer::default();
    let script = Script::from_file("C:\\Users\\mingm\\projects\\zmake\\test\\origin.ts");
    let script = transformer.transpile(script).unwrap();
    
    println!("\nOUTPUT:\n{}\nSOURCE_MAP:{}\n",script.transformed.clone().unwrap(),script.source_map.clone().unwrap_or(String::from("no source map")));

    let mut engine = V8Engine::default();

    engine.eval_module(&script);
}
