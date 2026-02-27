use anyhow::Result;
use bincode;
use std::fs;
use std::time::Instant;

use crate::{huffman, lz77, stats, ui};

pub fn compress_file(input: &str, output: &str) -> Result<()> {
    println!("▶ Compression started");
    let start_time = Instant::now();
    let data = fs::read(input)?;
    let original_size = data.len() as u64;

    // LZ77
    let spinner = ui::create_spinner("Running LZ77...");
    let lz = lz77::compress(&data);
    spinner.finish_with_message("✔ LZ77 done");

    // Serialize LZ77
    let serialized = bincode::serialize(&lz)?;

    // Huffman
    let spinner = ui::create_spinner("Running Huffman...");
    let (encoded, tree) = huffman::compress(&serialized);
    spinner.finish_with_message("✔ Huffman done");

    // Create compressed file
    let spinner = ui::create_spinner("Writing output file...");
    let final_data = bincode::serialize(&(original_size, encoded, tree))?;
    fs::write(output, &final_data)?;
    spinner.finish_with_message("✔ File written");

    let compressed_size = final_data.len() as u64;
    let duration = start_time.elapsed();

    stats::print_stats(original_size, compressed_size, duration);

    Ok(())
}
