pub fn print_stats(original: u64, compressed: u64) {
    let rate = 100.0 * (1.0 - (compressed as f64 / original as f64));

    println!("✔ Compression terminée");
    println!("Taille initiale      : {:.2} MB", original as f64 / 1_048_576.0);
    println!("Taille compressée    : {:.2} MB", compressed as f64 / 1_048_576.0);
    println!("Taux de compression  : {:.2} %", rate);
}