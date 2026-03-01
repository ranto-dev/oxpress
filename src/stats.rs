/**
 * affichage des statistiques deu compression
 */
use colored::Colorize;
use std::time::Duration;

pub fn print_stats(original_size: u64, compressed_size: u64, duration: Duration) {
    let original_mb = original_size as f64 / (1024.0 * 1024.0);
    let compressed_mb = compressed_size as f64 / (1024.0 * 1024.0);

    let ratio = 100.0 - ((compressed_size as f64 / original_size as f64) * 100.0);

    println!();
    println!("{}", "✔ Done".green());
    println!("──────────────────────────────────────");
    println!("🔹 Original size      : {:.2} MB", original_mb);
    println!("🔹 Compressed size    : {:.2} MB", compressed_mb);
    println!("🔹 Compression ratio  : {:.2} %", ratio);
    println!("🔹 Time elapsed        : {:.2?}", duration);
    println!("──────────────────────────────────────");
}
