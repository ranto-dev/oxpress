use crate::huffman;
use crate::lz77;
use bincode;
use std::fs;

pub fn compress_file(input: &str, output: &str) -> anyhow::Result<()> {
    let data = fs::read(input)?;

    let lz = lz77::compress(&data);
    let serialized = bincode::serialize(&lz)?;

    let (encoded, tree) = huffman::compress(&serialized);

    let final_data = bincode::serialize(&(data.len() as u64, encoded, tree))?;
    fs::write(output, final_data)?;

    Ok(())
}
