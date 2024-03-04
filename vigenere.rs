use std::io;

fn main() {
    println!("Vigenère Cipher Encryption and Decryption");

    // Get user input
    let mut input_text = String::new();
    println!("Enter the text to encrypt:");
    io::stdin().read_line(&mut input_text).expect("Failed to read input.");

    // Remove non-alphabetic characters and convert to uppercase
    let input_text = input_text.chars()
                                .filter(|&c| c.is_alphabetic())
                                .map(|c| c.to_ascii_uppercase())
                                .collect::<String>();

    // Get the key from the user
    println!("Enter the key (a word or phrase without non-alphabetic characters):");
    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Failed to read input.");

    // Remove non-alphabetic characters and convert to uppercase
    let key = key.chars()
                 .filter(|&c| c.is_alphabetic())
                 .map(|c| c.to_ascii_uppercase())
                 .collect::<String>();

    // Encrypt the input text
    let encrypted_text = encrypt(&input_text, &key);
    println!("Encrypted text: {}", encrypted_text);

    // Decrypt the encrypted text
    let decrypted_text = decrypt(&encrypted_text, &key);
    println!("Decrypted text: {}", decrypted_text);
}

// Function to encrypt text using Vigenère cipher
fn encrypt(text: &str, key: &str) -> String {
    let mut result = String::new();
    let mut key_iter = key.chars().cycle();
    for (i, c) in text.chars().enumerate() {
        if c.is_alphabetic() {
            let shift = key_iter.next().unwrap() as u8 - b'A';
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let encrypted_char = ((c as u8 - base) + shift) % 26 + base;
            result.push(encrypted_char as char);
        } else {
            result.push(c);
        }
    }
    result
}

// Function to decrypt text using Vigenère cipher
fn decrypt(text: &str, key: &str) -> String {
    let mut result = String::new();
    let mut key_iter = key.chars().cycle();
    for (i, c) in text.chars().enumerate() {
        if c.is_alphabetic() {
            let shift = key_iter.next().unwrap() as u8 - b'A';
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            let decrypted_char = ((c as u8 - base + 26 - shift) % 26 + base) as char;
            result.push(decrypted_char);
        } else {
            result.push(c);
        }
    }
    result
}
