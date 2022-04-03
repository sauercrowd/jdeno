use std::error::Error;
use deno_core::futures::FutureExt;
use deno_core::resolve_import;
use deno_core::ModuleLoader;
use deno_core::ModuleSource;
use std::rc::Rc;
use tokio::runtime::Builder;

pub struct MemoryLoader {
    files: std::collections::HashMap<String, String>,
}

impl ModuleLoader for MemoryLoader {
    fn resolve(
        &self,
        specifier: &str,
        referrer: &str,
        _is_main: bool,
    ) -> std::result::Result<deno_core::url::Url, deno_core::anyhow::Error> {
        Ok(resolve_import(specifier, referrer)?)
    }
    fn load(
        &self,
        module_specifier: &deno_core::url::Url,
        _maybe_referrer: std::option::Option<deno_core::url::Url>,
        _is_dynamic: bool,
    ) -> std::pin::Pin<
        std::boxed::Box<
            (dyn std::future::Future<
                Output = std::result::Result<deno_core::ModuleSource, deno_core::anyhow::Error>,
            > + 'static),
        >,
    > {
        let module_specifier = module_specifier.clone();
        let path = module_specifier.as_str();
        let files_clone = self.files.clone();
        println!("URL: {}", path);
        let code = files_clone.get(path).expect("No file with name found");
        let x = code.clone();
        async move {
            let module = ModuleSource {
                code: x,
                module_type: deno_core::ModuleType::JavaScript,
                module_url_specified: module_specifier.to_string(),
                module_url_found: module_specifier.to_string(),
            };
            Ok(module)
        }
        .boxed_local()
    }
}

#[test]

fn run_js() {
    let mut files = std::collections::HashMap::new();
    files.insert("file:///index".to_string(), std::fs::read_to_string("index.js").unwrap());
    files.insert("file:///lockbox.js".to_string(), std::fs::read_to_string("lockbox.js").unwrap());
    let module_loader = MemoryLoader { files: files };
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
        let _ = js_runtime.mod_evaluate(mod_id);
        js_runtime.run_event_loop(false).await?;
        Ok::<(), deno_core::anyhow::Error>(()) 
    };
    runtime.block_on(future).unwrap();
}

// pub fn create_runtime() {}