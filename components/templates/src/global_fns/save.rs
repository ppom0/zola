use std::collections::HashMap;
use std::path::PathBuf;

use utils::fs::{create_binary_file, create_file, ensure_directory_exists};

use libs::base64::engine::{general_purpose::STANDARD as standard_b64, Engine as _};
use libs::tera::{from_value, Function as TeraFn, Result, Value};

#[derive(Debug)]
pub struct SaveAsFile {
    output_path: PathBuf,
}

impl SaveAsFile {
    pub fn new(output_path: PathBuf) -> Self {
        Self { output_path }
    }
}

impl TeraFn for SaveAsFile {
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value> {
        let path = required_arg!(
            String,
            args.get("path"),
            "`save_as_file` requires a `path` argument with a string value"
        );
        let data = required_arg!(
            String,
            args.get("data"),
            "`save_as_file` requires a `data` argument with a string value"
        );
        let base64 = optional_arg!(
            bool,
            args.get("base64"),
            "`save_as_file` requires a `data` argument with a string value"
        );

        if path.contains("/..") {
            return Err("`save_as_file`\'s `path` argument must not contain reference to parent directory `..`".into());
        }
        let path = if path.starts_with('/') { ".".to_owned() + &path } else { path };

        let complete_path = &self.output_path.join(path);
        let complete_dir = complete_path.parent().unwrap();

        if let Err(err) = ensure_directory_exists(complete_dir) {
            return Err(err.to_string().into());
        };

        if base64.is_some_and(|b| b) {
            let data = standard_b64
                .decode(data.as_bytes())
                .map_err(|e| format!("`base64_decode`: failed to decode: {}", e))?;

            if let Err(err) = create_binary_file(complete_path, &data) {
                return Err(err.to_string().into());
            };
        } else {
            if let Err(err) = create_file(complete_path, &data) {
                return Err(err.to_string().into());
            };
        }

        Ok(Value::Bool(true))
    }

    fn is_safe(&self) -> bool {
        true
    }
}
