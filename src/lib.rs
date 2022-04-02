
use deno_core;

pub fn create_runtime(){
    let runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions::default());
    //println!(runtime.execute_script("", fs::read_to_string("index.js").unwrap()).unwrap());
}
