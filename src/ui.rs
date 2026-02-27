use colored::*;
use figlet_rs::FIGfont;

use indicatif::{ProgressBar, ProgressStyle};

pub fn print_banner() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("OXPRESS").unwrap();

    println!("{}", figure.to_string().bright_cyan());
    println!("{}", "Lossless Text Compression Tool".bright_cyan());
    println!(
        "{}",
        "LZ77 + Huffman Coding | Large Files Only (≥ 100 MB)\n".white()
    );
}

pub fn create_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();

    pb.set_style(ProgressStyle::with_template("{spinner} {msg}").unwrap());

    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    pb.set_message(message.to_string());

    pb
}
