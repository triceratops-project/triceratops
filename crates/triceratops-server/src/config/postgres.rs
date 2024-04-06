use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PostgresConfig {
    ip: IpAddr,
    port: u16,
    schema: String,
    username: String,
    password: Option<String>,
}

impl PostgresConfig {
    pub fn ip(&self) -> &IpAddr {
        &self.ip
    }

    pub fn port(&self) -> &u16 {
        &self.port
    }

    pub fn schema(&self) -> &String {
        &self.schema
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn password(&self) -> &Option<String> {
        &self.password
    }
}
