use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NatsConfig {
    /// NATS URL
    pub url: String,

    /// Subject to publish to
    pub subject: String,

    /// Messages namespace
    pub messages_namespace: String,
}
