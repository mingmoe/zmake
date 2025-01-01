use crate::get_absolute_path;

#[derive(Debug, Clone)]
pub struct Options {
    pub working_directory: String,
    pub cache_directory: String,
    pub zmake_directory: String,
    pub debug: bool,
}

impl Options {
    pub fn new(cache_directory: String, zmake_directory: String) -> Options {
        Options {
            working_directory: get_absolute_path("."),
            cache_directory,
            zmake_directory,
            debug: false,
        }
    }
}
