use serde::{Deserialize, Serialize};
use crate::config::Config;


#[derive(Serialize, Deserialize, Debug)]
pub struct Counter {
    pub count: i32
}


impl Counter {

    fn get_redis_url() -> String {
        let config = Config::new();
        let redis_url = config.map.get("REDIS_URL").unwrap().as_str().unwrap().to_owned();
        return redis_url 
    }

    pub fn save(self) -> Result<(), redis::RedisError> {
        let serialized = serde_yaml::to_vec(&self).unwrap();
        let client = match redis::Client::open(Counter::get_redis_url()) {
            Ok(client) => client,
            Err(error) => return Err(error)
        };
        let mut con = match client.get_connection() {
            Ok(con) => con,
            Err(error) => return Err(error)
        };
        match redis::cmd("SET").arg("COUNTER").arg(serialized).query::<Vec<u8>>(&mut con) {
            Ok(_) => Ok(()),
            Err(error) => Err(error)
        }
    }

    pub fn load() -> Result<Counter, redis::RedisError> {
        let client = match redis::Client::open(Counter::get_redis_url()){
            Ok(client) => client,
            Err(error) => return Err(error)
        };
        let mut con = match client.get_connection() {
            Ok(con) => con,
            Err(error) => return Err(error)
        };
        let byte_data: Vec<u8> = match redis::cmd("GET").arg("COUNTER").query(&mut con) {
            Ok(data) => data,
            Err(error) => return Err(error)
        };
        Ok(serde_yaml::from_slice(&byte_data).unwrap())
    }
}