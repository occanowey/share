use clap::Parser;
use color_eyre::Result;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, prelude::*};

pub fn setup<Args: Parser>() -> Result<Args> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    color_eyre::install()?;
    Ok(Args::parse())
}
