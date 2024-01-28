use std::net::{IpAddr, Ipv4Addr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename = "postgres")]
pub struct PostgresConfig {
    ip: IpAddr,
    port: u16,
    schema: String,
    username: String,
    password: Option<String>,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        let ip = IpAddr::from(Ipv4Addr::new(127, 0, 0, 1));
        
        Self {
            ip,
            port: 5432,
            schema: "triceratops".to_string(),
            username: "triceratops".to_string(),
            password: None,
        }
    }
}
