use crate::huffman;
use crate::lz77;
use bincode;
use std::fs;

pub fn decompress_file(input: &str, output: &str) -> anyhow::Result<()> {
    let bytes = fs::read(input)?;

    let (original_size, encoded, tree): (u64, Vec<u8>, huffman::HuffmanTree) =
        bincode::deserialize(&bytes)?;

    let decoded = huffman::decompress(&encoded, &tree);
    let lz_data: Vec<lz77::Token> = bincode::deserialize(&decoded)?;

    let mut result = lz77::decompress(&lz_data);
    result.truncate(original_size as usize);

    fs::write(output, result)?;

    Ok(())
}
