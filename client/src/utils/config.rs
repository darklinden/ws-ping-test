#[allow(non_camel_case_types, dead_code)]
pub enum ConfigKeys {
    WS_URL,
}

impl ConfigKeys {
    pub fn as_str(&self) -> &'static str {
        match self {
            ConfigKeys::WS_URL => "WS_URL",
        }
    }
}

pub fn get_config_str(key: ConfigKeys) -> String {
    let k = key.as_str();
    let value = dotenv::var(k);
    match value {
        Ok(value) => value,
        Err(_) => match key {
            ConfigKeys::WS_URL => {
                panic!("{} is not set in .env file", k);
            }
        },
    }
}

#[allow(dead_code)]
pub fn get_config<T>(key: ConfigKeys) -> Result<T, <T>::Err>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    return get_config_str(key).parse::<T>();
}
