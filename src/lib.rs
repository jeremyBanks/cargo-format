#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc
)]

use {
    eyre::{bail, Result},
    std::{env, process::Command},
    tracing::{instrument, warn},
    which::which,
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
    let env_cargo = env::var("CARGO").ok();
    let path_cargo = which("cargo").ok();
    let path_rustup = which("rustup").ok();
    let env_args = env::args().collect::<Vec<_>>();
    let as_subcommand = env_cargo.is_some() && env_args.len() >= 2 && env_args[1] == "format";
    let args = if as_subcommand {
        &env_args[2..]
    } else {
        &env_args[1..]
    };

    let env_cargo_is_nightly = env_cargo
        .as_ref()
        .and_then(|env_cargo| {
            let output = Command::new(env_cargo).arg("--version").output().ok()?;
            if output.status.success() {
                let stdout = std::str::from_utf8(&output.stdout).ok()?;
                Some(stdout.starts_with("cargo ") && stdout.contains("-nightly "))
            } else {
                None
            }
        })
        .unwrap_or(false);

    let path_cargo_is_nightly = path_cargo
        .as_ref()
        .and_then(|path_cargo| {
            let output = Command::new(path_cargo).arg("--version").output().ok()?;
            if output.status.success() {
                let stdout = std::str::from_utf8(&output.stdout).ok()?;
                Some(stdout.starts_with("cargo ") && stdout.contains("-nightly "))
            } else {
                None
            }
        })
        .unwrap_or(false);

    let rustup_has_nightly = path_rustup
        .as_ref()
        .and_then(|path_rustup| {
            let output = Command::new(path_rustup)
                .args(["run", "nightly", "cargo"])
                .arg("--version")
                .output()
                .ok()?;
            if output.status.success() {
                let stdout = std::str::from_utf8(&output.stdout).ok()?;
                Some(stdout.starts_with("cargo ") && stdout.contains("-nightly "))
            } else {
                None
            }
        })
        .unwrap_or(false);

    let mut command;

    let command = if env_cargo_is_nightly {
        command = Command::new(env_cargo.unwrap());
        &mut command
    } else if path_cargo_is_nightly {
        command = Command::new(path_cargo.unwrap());
        &mut command
    } else if rustup_has_nightly {
        command = Command::new(path_rustup.unwrap());
        command.args(["run", "nightly", "cargo"])
    } else {
        panic!("Rust nightly toolchain required, but not found in env or path");
    };

    let status = command.arg("fmt").args(args).status()?;
    if !status.success() {
        bail!("{:?} failed with {:?}", &command, &status)
    }

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
