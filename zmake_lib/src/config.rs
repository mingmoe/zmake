
#[derive(Debug, Clone)]
pub struct Options {
    pub source_directory: String,
    pub binary_directory: String,
    pub cache_directory: String,
    pub zmake_directory: String,
    pub debug: bool,
}
