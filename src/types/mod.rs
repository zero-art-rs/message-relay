mod centrifugo;
mod outbox;

pub use centrifugo::{CentrifugoEventType, CentrifugoMessage, CentrifugoMethod, CentrifugoPayload};
pub use outbox::MessageOutbox;

pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/zero_art_proto.rs"));
}