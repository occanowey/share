use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use clap::Parser;
use color_eyre::Result;
use tracing::info;

/// chunk
///
/// Convert a Simpsons Hit & Run save file from its non chunked format to a chunk format.
#[derive(Debug, Parser)]
#[command(about, version)]
struct Args {
    /// Path to a non chunked save file
    in_path: PathBuf,

    /// Path to write the chunked save
    out_path: PathBuf,
}

fn main() -> Result<()> {
    let args: Args = har::setup()?;

    let mut in_file = fs::File::open(args.in_path)?;

    let out_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(args.out_path)?;
    let mut writer = libhar::io::ChunkWriter::new(out_file);

    io::copy(&mut in_file, &mut writer)?;
    writer.flush()?;
    info!("Done");

    Ok(())
}
