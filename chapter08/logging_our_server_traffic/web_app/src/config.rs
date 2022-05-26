use std::collections::HashMap;
use std::env;
use serde_yaml;


/// Holds configuration parameters for the application
/// 
/// # Attributes
/// * map (HashMap<String, serde_yaml::Value>): the map of parameters for the app 
pub struct Config {
    pub map: HashMap<String, serde_yaml::Value>
}

impl Config {

    /// Creates a new Config map with loaded data from a yml file where the path is from the last argument 
    /// passed into the program.
    /// 
    /// # Arguments 
    /// None
    /// 
    /// # Returns 
    /// (Config): a loaded Config struct
    pub fn new() -> Config {
        let args: Vec<String> = env::args().collect();
        let file_path = &args[args.len() - 1];

        let file = std::fs::File::open(file_path).unwrap();
        let map: HashMap<String, serde_yaml::Value> = serde_yaml::from_reader(file).unwrap();
        return Config{map}
    }

}
