pub use error::{Error, ErrorResponse};

mod error;

/// Extra error conversions Casbin-Raft uses, if users want they can omit this feature to not pull in
/// hyper and tonic dependencies
#[cfg(feature = "extra-errors")]
mod extra_errors;
