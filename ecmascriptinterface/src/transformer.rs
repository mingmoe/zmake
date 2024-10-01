
use std::sync::Arc;
use swc::Compiler;
use swc_common::errors::{ColorConfig, Handler};
use swc_common::{FileName, SourceMap};
use crate::error::Error;

use crate::Script;

pub struct Transformer{
    compiler : Compiler,
    source_map: Arc<SourceMap>
}

impl Transformer{
    pub fn default() -> Transformer{
        let source_map = Arc::<SourceMap>::default();
        let compiler = swc::Compiler::new(source_map.clone());

        Transformer{
            compiler,
            source_map
        }
    }

    pub fn transpile(&self,script:Script) -> Result<Script,Error>{
        // https://github.com/swc-project/swc/discussions/4126
        let globals = swc_common::Globals::new();
        swc_common::GLOBALS.set(&globals,||{
            let handler = Handler::with_tty_emitter(
                ColorConfig::Auto,
                true,
                false,
                Some(self.source_map.clone()),
            );

            let fm = self
                .source_map
                .new_source_file(FileName::Custom(script.path.unwrap().clone()).into(), script.text.clone());

            let cfg_json = format!(
                r#"
            {{
              "minify": false,
              "sourceMaps": true,
              "module": {{
                    "type": "es6",
                    "strict": true,
                    "strictMode": true,
                    "lazy": false,
                    "noInterop": false,
                    "ignoreDynamic": true
                }},
              "jsc": {{
                "externalHelpers": false,
                "parser": {{
                  "syntax": "typescript",
                  "jsx": false,
                  "tsx": false,
                  "decorators": true,
                  "decoratorsBeforeExport": true,
                  "dynamicImport": true,
                  "preserveAllComments": false
                }},
                "transform": {{
                  "legacyDecorator": true,
                  "decoratorMetadata": true
                }},
                "target": "es2022",
                "keepClassNames": true
              }}
            }}

        "#);

            log::trace!("using config {}", cfg_json);

            let cfg = serde_json::from_str(cfg_json.as_str())
                .map_err(|e| Error::TransformeError { 
                    script:Script::from(cfg_json,String::from("the buillt-in swc configuration of zmake")),
                    reason: e.to_string()
                 })?;

            let ops = swc::config::Options {
                config: cfg,
                ..Default::default()
            };

            let res = self.compiler.process_js_file(fm, &handler, &ops);

            match res {
                Ok(to) => Ok(script.with_transformed(to.code,to.map)),
                Err(e) => Err(Error::TransformeError { 
                    script:script.clone(),
                    reason: e.to_string()
                 }),
            }
        })
    }
}
