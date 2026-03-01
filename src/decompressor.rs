/**
 * decompression controller logique
 */
use anyhow::Result;
use bincode;
use std::fs;

use crate::{huffman, lz77, ui};

pub fn decompress_file(input: &str, output: &str) -> Result<()> {
    println!("▶ Decompression started");

    // chargement du fichier compressé
    let bytes = fs::read(input)?;
    let spinner = ui::create_spinner("Reading archive...");
    let (original_size, encoded, tree): (u64, Vec<u8>, huffman::HuffmanTree) =
        bincode::deserialize(&bytes)?;
    spinner.finish_with_message("✔ Archive loaded");

    // décompression avec huffman
    let spinner = ui::create_spinner("Decoding Huffman...");
    let decoded = huffman::decompress(&encoded, &tree);
    spinner.finish_with_message("✔ Huffman decoded");

    // décompression avec lz77
    let spinner = ui::create_spinner("Decoding LZ77...");
    let lz_data: Vec<lz77::Token> = bincode::deserialize(&decoded)?;
    let mut result = lz77::decompress(&lz_data);
    spinner.finish_with_message("✔ LZ77 decoded");

    // restructuration et sauvegarde du fichier décompressé
    result.truncate(original_size as usize);
    let spinner = ui::create_spinner("Writing output...");
    fs::write(output, result)?;
    spinner.finish_with_message("✔ Decompression finished");

    Ok(())
}
