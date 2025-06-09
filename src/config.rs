pub use serde::Deserialize;

fn default_wsserver() -> String {
    "127.0.0.1".to_string()
}
fn default_port() -> u16 {
    3333
}

#[derive(Default, Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_wsserver")]
    pub ip: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

impl Config {
    pub fn load(path: String) -> Config {
        let content = std::fs::read_to_string("config.toml").unwrap();
        let settings: Config = toml::from_str(&content).unwrap();
        settings
    }
}
