extern crate des;

use std::io;

fn main() {
    println!("DES Encryption and Decryption");

    // Get user input
    let mut input_text = String::new();
    println!("Enter the text to encrypt:");
    io::stdin().read_line(&mut input_text).expect("Failed to read input.");

    // Remove trailing newline
    let input_text = input_text.trim();

    // Get the key from the user
    println!("Enter the key (8 bytes):");
    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Failed to read input.");

    // Remove trailing newline
    let key = key.trim();

    if key.len() != 8 {
        println!("Key length must be 8 bytes.");
        return;
    }

    // Convert the key to bytes
    let key_bytes = key.as_bytes();

    // Encrypt the input text
    let encrypted_text = des::encrypt(input_text.as_bytes(), key_bytes).unwrap();
    println!("Encrypted text: {:?}", encrypted_text);

    // Decrypt the encrypted text
    let decrypted_text = des::decrypt(&encrypted_text, key_bytes).unwrap();
    let decrypted_str = String::from_utf8(decrypted_text).unwrap();
    println!("Decrypted text: {}", decrypted_str);
}
