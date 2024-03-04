extern crate crypto;

use crypto::aes::{self, KeySize};
use crypto::blockmodes::NoPadding;
use crypto::buffer::{BufferResult, ReadBuffer, WriteBuffer};

use std::io;

fn main() {
    println!("AES Encryption and Decryption");

    // Get user input
    let mut input_text = String::new();
    println!("Enter the text to encrypt:");
    io::stdin().read_line(&mut input_text).expect("Failed to read input.");

    // Remove trailing newline
    let input_text = input_text.trim();

    // Get the key from the user
    println!("Enter the key (16, 24, or 32 bytes):");
    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Failed to read input.");

    // Remove trailing newline
    let key = key.trim();

    let key_size = match key.len() {
        16 => KeySize::KeySize128,
        24 => KeySize::KeySize192,
        32 => KeySize::KeySize256,
        _ => {
            println!("Key length must be 16, 24, or 32 bytes.");
            return;
        }
    };

    // Convert the key to bytes
    let key_bytes = key.as_bytes();

    // Encrypt the input text
    let encrypted_text = encrypt(input_text, key_bytes, key_size).unwrap();
    println!("Encrypted text: {:?}", encrypted_text);

    // Decrypt the encrypted text
    let decrypted_text = decrypt(&encrypted_text, key_bytes, key_size).unwrap();
    let decrypted_str = String::from_utf8(decrypted_text).unwrap();
    println!("Decrypted text: {}", decrypted_str);
}

// Function to encrypt text using AES
fn encrypt(input: &str, key: &[u8], key_size: KeySize) -> Result<Vec<u8>, aes::Error> {
    let mut encryptor = aes::cbc_encryptor(
        key_size,
        key,
        &[0u8; 16], // Initialization vector (IV), can be any 16-byte value
        NoPadding,
    );

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(input.as_bytes());
    let mut buffer = [0; 4096];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {},
        }
    }

    Ok(final_result)
}

// Function to decrypt text using AES
fn decrypt(encrypted: &[u8], key: &[u8], key_size: KeySize) -> Result<Vec<u8>, aes::Error> {
    let mut decryptor = aes::cbc_decryptor(
        key_size,
        key,
        &[0u8; 16], // Initialization vector (IV), must be the same as the one used for encryption
        NoPadding,
    );

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(encrypted);
    let mut buffer = [0; 4096];
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)?;
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {},
        }
    }

    Ok(final_result)
}
