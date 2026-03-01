/**
 * compression controller logique
 */

use anyhow::{Result, bail};
use bincode;
use std::fs;
use std::path::Path;
use std::time::Instant;

use crate::{huffman, lz77, stats, ui};

const MIN_SIZE_BYTES: u64 = 100 * 1024 * 1024; // pour 100 MB

pub fn compress_file(input: &str, output: &str) -> Result<()> {
    println!("▶ Compression started");

    let start_time = Instant::now();
    let data = fs::read(input)?;
    let original_size = data.len() as u64;
    let output_path = Path::new(output);

    // verification de la taille du fichier input pour la compression
    if original_size < MIN_SIZE_BYTES {
        bail!(
            "File too small. Minimum required size is 100 MB.\nCurrent size: {:.2} MB",
            original_size as f64 / (1024.0 * 1024.0)
        );
    }

    // verification pour le fichier output du compression
    match output_path.extension() {
        Some(ext) if ext == "oxp" => {}
        _ => bail!("Output file must have '.oxp' extension."),
    }

    // compression avec LZ77
    let spinner = ui::create_spinner("Running LZ77...");
    let lz = lz77::compress(&data);
    spinner.finish_with_message("✔ LZ77 done");

    // Serialize LZ77
    let serialized = bincode::serialize(&lz)?;

    // compression avec Huffman
    let spinner = ui::create_spinner("Running Huffman...");
    let (encoded, tree) = huffman::compress(&serialized);
    spinner.finish_with_message("✔ Huffman done");

    // création du fichier compressé
    let spinner = ui::create_spinner("Writing output file...");
    let final_data = bincode::serialize(&(original_size, encoded, tree))?;
    fs::write(output, &final_data)?;
    spinner.finish_with_message("✔ File written");

    // affichage des statistiques de lq compression
    let compressed_size = final_data.len() as u64;
    let duration = start_time.elapsed();
    stats::print_stats(original_size, compressed_size, duration);

    Ok(())
}
