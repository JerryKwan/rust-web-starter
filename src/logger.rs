// logger related functions
use std::env;
use anyhow::{Context, Ok};
use tracing::subscriber;
use std::fs::File;
use tracing::Level;
use tracing_subscriber::{fmt, Registry, layer::SubscriberExt};
use tracing_subscriber::{layer, EnvFilter};
use tracing_appender::rolling::{Rotation, RollingFileAppender};
use tracing_subscriber::prelude::*;
use tracing_subscriber::fmt::time::UtcTime;
use std::path::Path;

pub fn setup_logging_and_tracing(file_name: &str) -> anyhow::Result<tracing_appender::non_blocking::WorkerGuard> {
    // check file directory and create if not exists
    let path = Path::new(file_name);
    let parent = path.parent().unwrap();
    match parent.exists() {
        true => (),
        false => std::fs::create_dir_all(parent).unwrap(),
    }
    // file appender
    // let file_appender = tracing_appender::rolling::daily(
    //     parent,
    //     path.file_name().unwrap(),
    // );
    let file_appender = RollingFileAppender::builder()
        .max_log_files(7)
        .rotation(Rotation::DAILY)
        .filename_prefix(path.file_name().unwrap().to_str().unwrap())
        .build(parent)
        .expect("failed to initialize rolling file appender");
    let (file_writer, _guard) = tracing_appender::non_blocking(file_appender);
    let env_filter = env::var("RUST_LOG")
        .map(|_| EnvFilter::from_default_env())
        .or_else(|_| EnvFilter::try_new(format!("DEBUG")))
        .context("failed to set up tracing env filter")?;
    let mut layers = Vec::new();
    let event_format = tracing_subscriber::fmt::format()
        .with_target(true)
        .with_timer(
            // We do not rely on the Rfc3339 implementation, because it has a nanosecond precision.
            // See discussion here: https://github.com/time-rs/time/discussions/418
            UtcTime::new(
                time::format_description::parse(
                    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z",
                )
                .expect("Time format invalid."),
            ),
        );
    let layer = tracing_subscriber::fmt::layer()
        .event_format(event_format)
        .boxed();
    layers.push(layer);

    let event_format_file = tracing_subscriber::fmt::format()
        .with_target(true)
        .with_ansi(false)
        .with_timer(
            // We do not rely on the Rfc3339 implementation, because it has a nanosecond precision.
            // See discussion here: https://github.com/time-rs/time/discussions/418
            UtcTime::new(
                time::format_description::parse(
                    "[year]-[month]-[day]T[hour]:[minute]:[second].[subsecond digits:3]Z",
                )
                .expect("Time format invalid."),
            ),
        );
    let layer = tracing_subscriber::fmt::layer()
        .event_format(event_format_file)
        .with_writer(file_writer)
        .boxed();
    layers.push(layer);

    let registry = tracing_subscriber::registry().with(env_filter);
    // layers
    registry
    .with(layers)
    .try_init()
    .context("failed to register tracing subscriber")?;
    Ok(_guard)
}