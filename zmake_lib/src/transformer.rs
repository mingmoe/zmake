use crate::error::Error;
use crate::Script;
use crate::ScriptType;
use std::sync::Arc;
use swc::config::Config;
use swc::Compiler;
use swc_common::errors::{ColorConfig, Handler};
use swc_common::{FileName, SourceMap};

pub struct Transformer {
    compiler: Compiler,
    source_map: Arc<SourceMap>,
    options: swc::config::Options,
}

impl Transformer {
    pub fn default() -> Transformer {
        let source_map = Arc::<SourceMap>::default();
        let compiler = swc::Compiler::new(source_map.clone());

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

    "#
        );

        let config: Config = serde_json::from_str(cfg_json.as_str()).unwrap();

        let options = swc::config::Options {
            config: config,
            ..Default::default()
        };

        Transformer {
            compiler,
            source_map,
            options,
        }
    }

    pub fn transpile(&self, script: Script) -> Result<Script, Error> {
        // https://github.com/swc-project/swc/discussions/4126
        if script.text_type == ScriptType::Ecmascript {
            return Ok(script);
        }

        let globals = swc_common::Globals::new();
        swc_common::GLOBALS.set(&globals, || {
            let handler = Handler::with_tty_emitter(
                ColorConfig::Auto,
                true,
                false,
                Some(self.source_map.clone()),
            );

            let fm = self.source_map.new_source_file(
                FileName::Custom(script.path.as_ref().unwrap().clone()).into(),
                script.get_text(),
            );

            let res = self.compiler.process_js_file(fm, &handler, &self.options);

            match res {
                Ok(to) => Ok(script.to_transformed(to.code, to.map)),
                Err(e) => Err(Error::TransformeError {
                    script: script,
                    reason: e.to_string(),
                }),
            }
        })
    }
}
