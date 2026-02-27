use colored::*;
use figlet_rs::FIGfont;

pub fn print_banner() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("PYCODEC").unwrap();

    println!("{}", figure.to_string().bright_cyan());
    println!("{}", "Lossless Text Compression Tool".white());
    println!("{}", "LZ77 • Huffman Coding".green());
    println!(
        "{}",
        "Academic Project • Large Files Only (≥ 100 MB)\n".dimmed()
    );
}
