use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RedisConfig {
    ip: IpAddr,
    port: u16,
    password: Option<String>,
    expiry: i64,
    max_connections: usize,
}

impl RedisConfig {
    pub fn ip(&self) -> &IpAddr {
        &self.ip
    }

    pub fn port(&self) -> &u16 {
        &self.port
    }

    pub fn password(&self) -> &Option<String> {
        &self.password
    }

    pub fn expiry(&self) -> &i64 {
        &self.expiry
    }

    pub fn max_connections(&self) -> &usize {
        &self.max_connections
    }
}
