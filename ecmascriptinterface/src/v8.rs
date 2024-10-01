
use std::{cell::Cell, rc::Rc, sync::Arc};
use crate::Script;
use v8::{self, Handle, Local, OwnedIsolate, Platform, Promise, SharedRef};

pub struct V8Engine<'a>{
    platform:SharedRef<Platform>,
    isolate:Box<OwnedIsolate>,
    modules:Vec<Local<'a,v8::Module>>,
}

impl  V8Engine<'_> {
    pub fn default() -> V8Engine<'static>{
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform.clone());
        v8::V8::initialize();

        let params = v8::CreateParams::default();

        let mut isolate = Box::from(v8::Isolate::new(params));

        isolate.set_capture_stack_trace_for_uncaught_exceptions(true,16);
        isolate.set_host_import_module_dynamically_callback(callback);

        return V8Engine{
            platform,
            isolate,
            modules:Vec::new()
        }
    }

    pub fn eval_module_and_dependency(&mut self,script:&Script){
        let scope = &mut v8::HandleScope::new(&mut *self.isolate);
        let context = v8::Context::new(scope, Default::default());
        let scope = &mut v8::ContextScope::new(scope, context);

        let code = v8::String::new(scope, script.transformed.as_ref().unwrap_or(&script.text)).unwrap();
        println!("javascript code: {}", code.to_rust_string_lossy(scope));
        
        let resource_name = v8::String::new(scope, "from.mjs").unwrap();
        let origin = v8::ScriptOrigin::new(
            scope,
             resource_name.cast(), 
             0, 
             0, 
             false,
             0, 
             None,
             false, 
             false,
             true, 
             None);
            

        let mut source: v8::script_compiler::Source = v8::script_compiler::Source::new(code, Some(&origin));
        let module = v8::script_compiler::compile_module(scope, &mut source).unwrap();

        self.modules.push(module);

        let requests = module.get_module_requests();
        let data = requests.get(scope, 0).unwrap();
        let a = data.try_cast::<v8::ModuleRequest>().unwrap();
        
        module.instantiate_module(scope, 
            |context,
            string,
            array,
            module| 
            {
                return Some(module); 
            }).unwrap();
            
        let result = module.evaluate(scope).unwrap();
        let promise = result.cast::<Promise>();
        // module.get_module_requests()
        let result = promise.result(scope).to_string(scope).unwrap();
        println!("result: {}", result.to_rust_string_lossy(scope));
    }

}