use crate::memory_loader::MemoryLoader;
use core::ptr::NonNull;
use std::rc::Rc;
use v8::Value;

mod memory_loader;

#[test]

fn run_js() {
    // let mut files = std::collections::HashMap::new();
    // files.insert(
    //     "index".to_string(),
    //     std::fs::read_to_string("index.js").unwrap(),
    // );
    // files.insert(
    //     "lockbox.js".to_string(),
    //     std::fs::read_to_string("lockbox.js").unwrap(),
    // );

    // let module_loader = memory_loader::MemoryLoader { files: files };

    let mut module_loader = MemoryLoader::new();
    module_loader.add_module(
        "lockbox.js",
        &std::fs::read_to_string("lockbox.js").unwrap(),
    );
    module_loader.add_module("index", &std::fs::read_to_string("index.js").unwrap());

    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(module_loader)),
        ..deno_core::RuntimeOptions::default()
    });

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let main_module = deno_core::resolve_path("/index").unwrap();

    let future = async move {
        let mod_id = js_runtime.load_main_module(&main_module, None).await?;
        let mut x = js_runtime.mod_evaluate(mod_id);
        println!("{:#?}", x.try_recv().expect(""));
        js_runtime.run_event_loop(false).await?;

        let r = js_runtime
            .execute_script("main.js", "globalThis.i")
            .expect("errror!!!!");
        let scope = &mut js_runtime.handle_scope();
        let local = v8::Local::new(scope, r);
        let deserialised_value = serde_v8::from_v8::<serde_json::Value>(scope, local);
        println!("r, {}", deserialised_value.unwrap());
        Ok::<(), deno_core::anyhow::Error>(())
    };

    runtime.block_on(future).unwrap();
    // let x = NonNull::<&str>::dangling();
    // v8::String::from_str()
    // let global = v8::Global::from_raw(js_runtime.v8_isolate(), "".as_bytes());
    // js_runtime.resolve_value(global);
    // js_runtime.poll_value(global, )
}
