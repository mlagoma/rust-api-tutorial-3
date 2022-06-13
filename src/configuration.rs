use std::env;
use regex::Regex;
use regex::Captures;
use std::borrow::Cow;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let mut settings = config::Config::default();

    // Add configuration values from a file named `configuration`.
    // It will look for any top-level file with an extension
    // that `config` knows how to parse: yaml, json, etc.
    let raw_config = config::File::with_name("configuration").try_into(String);
    settings.merge(expand_var(raw_config))?;

    // Try to convert the configuration values it read into
    // our Settings type
    settings.try_into()
}

fn expand_var(raw_config: &str) -> Cow<str> {

    let re = Regex::new(r"\$\{([a-zA-Z_][0-9a-zA-Z_]*)\}").unwrap();
    re.replace_all(&raw_config, |caps: &Captures| {
        match env::var(&caps[1]) {
            Ok(val) => val,
            Err(_) => (&caps[0]).to_string(),
        }
    })
}
