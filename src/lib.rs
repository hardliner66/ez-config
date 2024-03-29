use std::path::Path;
use std::fs::File;
use std::io::Read;
use serde::de::DeserializeOwned;
use std::io::Write;
use serde::Serialize;

#[derive(Debug)]
pub enum ConfigError {
    IoError(std::io::Error),
    ParseError(Box<std::error::Error>),
}

fn read(file: &mut File) -> std::result::Result<String, std::io::Error> {
    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => Ok(content),
        Err(e) => Err(e),
    }
}

#[cfg(feature = "default")]
mod serializer {
    pub use serde_json::from_str as deserialize;
    pub use serde_json::to_string_pretty as serialize;
}

#[cfg(all(not(feature = "default"), feature = "use-toml"))]
mod serializer {
    pub use toml::from_str as deserialize;
    pub use toml::to_string_pretty as serialize;
}

pub fn load_config<PathLike, Config>(path: PathLike) -> Result<Config, ConfigError>
    where
        Config: DeserializeOwned,
        PathLike: AsRef<Path>,
{
    let file = path.as_ref();
    match File::open(&file) {
        Ok(mut f) => match read(&mut f) {
            Ok(content) => match serializer::deserialize(&content) {
                Ok(c) => Ok(c),
                Err(e) => Err(ConfigError::ParseError(e.into())),
            }
            Err(e) => Err(ConfigError::IoError(e)),
        },
        Err(e) => Err(ConfigError::IoError(e)),
    }
}

pub fn save_config<PathLike, Config>(path: PathLike, config: &Config) -> Result<(), std::io::Error>
    where
        Config: Serialize,
        PathLike: AsRef<Path>,
{
    File::create(path)?.write_all(&serializer::serialize(&config).unwrap().as_bytes())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
