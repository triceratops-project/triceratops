use std::net::IpAddr;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RedisConfig {
    ip: IpAddr,
    port: u16,
    password: Option<String>,
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
}
