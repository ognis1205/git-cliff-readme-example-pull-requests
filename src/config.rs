#[derive(Debug)]
pub enum Format {
    Json,
    Toml,
    Yaml,
    Unknown,
}

#[derive(Debug)]
pub struct Config {
    pub path: String,
    pub format: Format,
}

impl Config {
    pub fn new(path: &str) -> Self {
        let format = if path.ends_with(".json") {
            Format::Json
        } else if path.ends_with(".toml") {
            Format::Toml
        } else if path.ends_with(".yaml") || path.ends_with(".yml") {
            Format::Yaml
        } else {
            Format::Unknown
        };
        Self {
            path: path.to_string(),
            format,
        }
    }

    pub fn load(&self) {
        println!("Loading config from {} as {:?}", self.path, self.format);
    }
}
