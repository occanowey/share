use std::{fs, io, path::PathBuf};

use clap::Parser;
use color_eyre::Result;
use tracing::info;

/// unchunk
///
/// Convert a Simpsons Hit & Run save file from its chunked format to a non chunk format.
#[derive(Debug, Parser)]
#[command(about, version)]
struct Args {
    /// Path to a chunked save file
    in_path: PathBuf,

    /// Path to write the non chunked save
    out_path: PathBuf,
}

fn main() -> Result<()> {
    let args: Args = har::setup()?;

    let in_file = fs::File::open(args.in_path)?;
    let mut reader = libhar::io::ChunkReader::new(in_file);

    let mut out_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(args.out_path)?;

    io::copy(&mut reader, &mut out_file)?;
    info!("Done");

    Ok(())
}
