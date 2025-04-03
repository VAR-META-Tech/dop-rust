use deno_core::v8;
use deno_core::{
    FsModuleLoader, JsRuntime, ModuleSpecifier, PollEventLoopOptions, RuntimeOptions,
    error::AnyError,
};
use std::path::PathBuf;
use std::rc::Rc;

pub struct TsLib {
    runtime: JsRuntime,
    main_module_id: deno_core::ModuleId,
    module_url: ModuleSpecifier,
}

impl TsLib {
    pub async fn new_from_path(path: &str) -> Result<Self, AnyError> {
        // Convert the path into an absolute module specifier
        let module_path = PathBuf::from(path).canonicalize()?;
        let module_url = ModuleSpecifier::from_file_path(&module_path)
            .map_err(|_| AnyError::msg("Invalid module path"))?;

        // Set up module loader and runtime
        let loader = Rc::new(FsModuleLoader);
        let mut runtime = JsRuntime::new(RuntimeOptions {
            module_loader: Some(loader),
            ..Default::default()
        });

        // Load the main module
        let module_id = runtime.load_main_es_module(&module_url).await?;
        let mod_eval = runtime.mod_evaluate(module_id);
        runtime
            .run_event_loop(PollEventLoopOptions::default())
            .await?;
        mod_eval.await?;

        Ok(Self {
            runtime,
            main_module_id: module_id,
            module_url,
        })
    }

    pub async fn greet(&mut self, name: &str) -> Result<String, AnyError> {
        let script = format!(
            "(async () => (await import('{}')).greet('{}'))()",
            self.module_url, name
        );

        let promise = self.runtime.execute_script("call_greet.js", script)?;

        // ✅ Await the returned Promise
        let resolved = self.runtime.resolve_value(promise).await?;

        let scope = &mut self.runtime.handle_scope();
        let local = v8::Local::new(scope, &resolved);
        Ok(local.to_rust_string_lossy(scope))
    }
}
