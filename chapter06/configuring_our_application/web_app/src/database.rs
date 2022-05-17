use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::config::Config;


pub fn establish_connection() -> PgConnection {
    let database_url = Config::new().map.get("DB_URL").unwrap().as_str().unwrap().to_string();
    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
