
use deno_core::futures::FutureExt;
use deno_core::resolve_import;
use deno_core::ModuleLoader;
use deno_core::ModuleSource;

pub struct MemoryLoader {
    pub files: std::collections::HashMap<String, String>,
}

impl MemoryLoader{
    pub fn new() -> MemoryLoader {
        return MemoryLoader{
            files: std::collections::HashMap::new()
        }
    }
    pub fn add_module(&mut self, name: &str, code: &str){
        self.files.insert("file:///".to_owned()+&name.to_string(), code.to_string());
        println!("file:///{}", name)
    }
    // fn run_module()
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
        // let mut key = "file:///".to_owned();
        // key.push_str(path);
        // println!("key: {}, {:#?}", &key, files_clone.keys());
        let code = files_clone.get(path).expect(&("No file with name found: ".to_owned() + path));
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
