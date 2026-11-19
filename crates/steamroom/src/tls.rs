//! Process-wide rustls [`CryptoProvider`] installation.
//!
//! reqwest is built with `rustls-no-provider` and tokio-tungstenite's rustls
//! dependency has no provider baked in, so both build their `ClientConfig` from
//! the process-default provider. That default must be installed before the
//! first TLS connection. The provider is chosen at compile time by the `ring`
//! (pure Rust) or `aws-lc-rs` (links C) feature.

use std::sync::Once;

static INSTALL: Once = Once::new();

/// Install the compile-time-selected rustls crypto provider as the process
/// default, at most once. Idempotent; safe to call from every connection path.
///
/// With neither feature enabled this does nothing: the host application installs the provider
/// (nie installs rustls-graviola), and `ClientConfig` builders read the process default.
///
/// `install_default` returns `Err` when a default is already set (for example
/// installed by the host application); that is the intended outcome and is
/// ignored.
pub fn ensure_crypto_provider() {
    INSTALL.call_once(|| {
        #[cfg(feature = "ring")]
        let _ = rustls::crypto::ring::default_provider().install_default();
        #[cfg(all(feature = "aws-lc-rs", not(feature = "ring")))]
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    });
}
