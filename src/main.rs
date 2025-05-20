use std::{fs, io::Read, path::PathBuf};

use clap::Parser;
use color_eyre::Result;
use tracing::info;

/// har
///
/// Parse The Simpsons Hit & Run save files.
#[derive(Debug, Parser)]
#[command(about, version)]
struct Args {
    /// Path to a save file
    save_path: PathBuf,
}

fn main() -> Result<()> {
    let args: Args = har::setup()?;
    dbg!(&args);

    let file = fs::File::open(args.save_path)?;
    let mut reader = libhar::io::ChunkReader::new(file);

    let mut read_buffer = [0u8; 512];
    let mut total_len = 0;
    loop {
        let len = reader.read(&mut read_buffer)?;
        if len == 0 {
            break;
        }

        info!("read {} bytes", len);
        total_len += len;
    }

    info!("total bytes: {}", total_len);

    Ok(())
}
