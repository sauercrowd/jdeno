
use deno_core;

pub fn create_runtime(){
    let runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions::default());
}
