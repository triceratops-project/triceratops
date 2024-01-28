use std::net::{IpAddr, Ipv4Addr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WebServerConfig {
    bind_address: IpAddr,
    port: u16,
}

impl Default for WebServerConfig {
    fn default() -> Self {
        let ip_v4 = Ipv4Addr::new(0, 0, 0, 0);

        Self {
            port: 8080,
            bind_address: IpAddr::from(ip_v4),
        }
    }
}
