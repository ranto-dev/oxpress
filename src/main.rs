/**
 * point d'entree du programme
 */
mod compressor;
mod decompressor;
mod huffman;
mod lz77;
mod stats;
mod ui;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "PYCODEC")]
#[command(about = "Lossless Text Compression Tool (LZ77 + Huffman)")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compress { input: String, output: String },
    Decompress { input: String, output: String },
}

fn main() -> Result<()> {
    ui::print_banner();

    let cli = Cli::parse();

    match cli.command {
        Commands::Compress { input, output } => {
            compressor::compress_file(&input, &output)?;
        }
        Commands::Decompress { input, output } => {
            decompressor::decompress_file(&input, &output)?;
        }
    }

    Ok(())
}
