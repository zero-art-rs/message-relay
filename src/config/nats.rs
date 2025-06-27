use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NatsConfig {
    /// NATS URL
    pub url: String,
}
