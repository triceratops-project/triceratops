use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum LogLevel {
    Info,
    Warn,
    Debug,
    Error,
    Trace,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DebugConfig {
    #[serde(rename = "log-level")]
    log_level: LogLevel,
}
