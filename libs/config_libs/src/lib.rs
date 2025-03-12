use config::{Config, ConfigError, Environment, File};
use serde::de::DeserializeOwned;
use dotenv::dotenv;

pub fn libs_config<T>(config_path: &str,prefix: &str) -> Result<T,ConfigError>
where
    T: DeserializeOwned
{
    dotenv().ok();

    let settings= match Config::builder()
                .add_source(File::with_name(config_path))
                .add_source(
                    Environment::with_prefix(prefix).separator("_"),
                )
                .build(){
                    Ok(settings)=>settings,
                    Err(error)=>return Err(error)
                };

    let deserialized = settings.try_deserialize();

    match deserialized {
        Ok(config)=>{
            return Ok(config)
        },
        Err(error)=>{
            return Err(error)
        }
    }
}
