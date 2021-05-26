//! Implementation of http transport

#[cfg(feature = "blocking")]
mod blocking;
#[cfg(feature = "blocking")]
pub use blocking::Http;

#[cfg(feature = "non-blocking")]
mod non_blocking;
#[cfg(feature = "non-blocking")]
pub use non_blocking::Http as AsyncHttp;
