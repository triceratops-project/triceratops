use std::net::IpAddr;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct WebServerConfig {
    url: Url,
    bind_address: IpAddr,
    port: u16,
    headless: Option<bool>,
}

impl WebServerConfig {
    pub fn url(&self) -> &Url {
        &self.url
    }

    pub fn bind_address(&self) -> &IpAddr {
        &self.bind_address
    }

    pub fn port(&self) -> &u16 {
        &self.port
    }

    pub fn headless(&self) -> &Option<bool> {
        &self.headless
    }
}
