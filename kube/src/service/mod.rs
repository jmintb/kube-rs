//! Abstracts the connection to Kubernetes API server.

mod auth;
#[cfg(feature = "gzip")] mod compression;
mod headers;
mod log;
mod url;

#[cfg(feature = "gzip")]
pub(crate) use self::compression::{accept_compressed, maybe_decompress};
pub use self::url::set_cluster_url;
pub(crate) use self::{
    auth::{AuthLayer, Authentication},
    headers::set_default_headers,
    log::LogRequest,
};
