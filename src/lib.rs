//! Schema-derived meta signal contract for privileged Listener configuration.
//!
//! Ordinary capture and transcription traffic lives in `signal-listener`.
//! This crate carries owner-only configuration over the Listener meta socket.

#[rustfmt::skip]
pub mod schema;

pub use schema::lib::*;

impl ConfigurationGeneration {
    pub fn value(&self) -> u64 {
        *self.payload()
    }
}

impl Input {
    pub fn kind(&self) -> OperationKind {
        match self {
            Self::Configure(_) => OperationKind::Configure,
        }
    }
}

pub type Operation = Input;
pub type MetaListenerOperation = Input;
pub type MetaListenerReply = Output;
