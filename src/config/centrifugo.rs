use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CentrifugoConfig {
    /// AMQP URL
    pub amqp_url: String,

    /// Centrifugo URL
    pub centrifugo_url: String,

    /// Centrifugo API key
    pub centrifugo_api_key: String,
}
