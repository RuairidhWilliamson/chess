use std::fs::File;
use std::io::BufReader;
use serde::{Serialize, Deserialize};

const ENV_PATH: &str = "config.json";


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub lichess: LichessConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LichessConfig {
    pub api_key: String,
    pub my_id: String,
    pub base_url: String,
    pub challenge_filter: ChallengeFilter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChallengeFilter {
    pub variant_whitelist: Vec<String>,
    pub time_control_whitelist: Vec<String>,
}

pub fn load_env() -> std::io::Result<Config> {
    let file = open_env()?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;
    Ok(config)
}

fn open_env() -> std::io::Result<File> {
    File::open(ENV_PATH).or_else(|e| {
        create_env();
        Err(e)
    })
}

fn create_env() -> File {
    File::create(ENV_PATH).expect("Couldn't create create .env")
}
