use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "App::default")]
    pub app: App,
}

#[derive(Deserialize, Debug)]
pub struct App {
    pub log_level: String,
}
impl App {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
        }
    }
}

pub fn from_env() -> Result<Config, envy::Error> {
    envy::from_env::<Config>()
}
