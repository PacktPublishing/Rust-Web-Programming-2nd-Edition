use std::collections::HashMap;
use std::env;

use serde_yaml;


pub struct Config {
    pub map: HashMap<String, serde_yaml::Value>
}


impl Config {

    // existing function reading from file
    #[cfg(not(test))]
    pub fn new() -> Config {
        let args: Vec<String> = env::args().collect();
        let file_path = &args[args.len() - 1];

        let file = std::fs::File::open(file_path).unwrap();
        let map: HashMap<String, serde_yaml::Value> = serde_yaml::
        from_reader(file).unwrap();
        return Config{map}
    }

    // new function for testing
    #[cfg(test)]
    pub fn new() -> Config {
        let mut map = HashMap::new();
        map.insert(String::from("DB_URL"),
                   serde_yaml::from_str(
                       "postgres://username:password@localhost:5433/to_do")
                       .unwrap());
        map.insert(String::from("SECRET_KEY"),
                   serde_yaml::from_str("secret").unwrap());
        map.insert(String::from("EXPIRE_MINUTES"),
                   serde_yaml::from_str("120").unwrap());
        map.insert(String::from("REDIS_URL"),
                   serde_yaml::from_str("redis://127.0.0.1/").unwrap());
        return Config {map}

    }
}

