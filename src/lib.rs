#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc
)]

use {
    eyre::Result,
    std::{env},
    tracing::{instrument, warn},
};

/// CLI entry point.
///
/// # Panics
///
/// For some fatal errors.
///
/// # Errors
///
/// For other fatal errors.
#[instrument(level = "debug")]
pub fn main() -> Result<()> {
    let env_args = env::args().collect::<Vec<_>>();
    let env_cargo = env::var("CARGO").unwrap_or_default();
    dbg!(env_args, env_cargo);

    Ok(())
}

/// Initialize the typical global environment for cargo-format's [main] CLI
/// entry point.
///
/// # Panics
///
/// This will panic if called multiple times, or if other code attempts
/// conflicting global initialization of systems such as logging.
pub fn init() {
    color_eyre::install().unwrap();

    let log_env = env::var("RUST_LOG").unwrap_or_default();

    let log_level = if !log_env.is_empty() {
        log_env
    } else {
        "warn".to_string()
    };

    tracing_subscriber::util::SubscriberInitExt::init(tracing_subscriber::Layer::with_subscriber(
        tracing_error::ErrorLayer::default(),
        tracing_subscriber::fmt()
            .with_env_filter(::tracing_subscriber::EnvFilter::new(log_level))
            .with_target(false)
            .with_span_events(
                tracing_subscriber::fmt::format::FmtSpan::ENTER
                    | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
            )
            .compact()
            .finish(),
    ));
}
