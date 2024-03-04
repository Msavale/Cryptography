use std::io;

fn main() {
    println!("Transposition Cipher Encryption and Decryption");

    // Get user input
    let mut input_text = String::new();
    println!("Enter the text to encrypt:");
    io::stdin().read_line(&mut input_text).expect("Failed to read input.");

    // Remove trailing newline
    input_text = input_text.trim().to_string();

    // Get the key from the user
    println!("Enter the key (a permutation of numbers representing column order):");
    let mut key_text = String::new();
    io::stdin().read_line(&mut key_text).expect("Failed to read input.");

    // Remove trailing newline and convert key to vector of usize
    let key: Vec<usize> = key_text
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Not a valid number"))
        .collect();

    // Encrypt the input text
    let encrypted_text = encrypt(&input_text, &key);
    println!("Encrypted text: {}", encrypted_text);

    // Decrypt the encrypted text
    let decrypted_text = decrypt(&encrypted_text, &key);
    println!("Decrypted text: {}", decrypted_text);
}

// Function to encrypt text using transposition cipher
fn encrypt(text: &str, key: &[usize]) -> String {
    let mut result = String::new();
    let cols = key.len();
    let rows = (text.len() as f64 / cols as f64).ceil() as usize;

    let mut matrix: Vec<Vec<char>> = vec![vec![' '; cols]; rows];

    for (i, c) in text.chars().enumerate() {
        matrix[i / cols][i % cols] = c;
    }

    for &col in key {
        for row in &matrix {
            if let Some(&c) = row.get(col) {
                result.push(c);
            }
        }
    }

    result
}

// Function to decrypt text using transposition cipher
fn decrypt(text: &str, key: &[usize]) -> String {
    let mut result = String::new();
    let cols = key.len();
    let rows = (text.len() as f64 / cols as f64).ceil() as usize;

    let mut matrix: Vec<Vec<Option<char>>> = vec![vec![None; cols]; rows];

    let mut index = 0;
    for &col in key {
        let mut row_index = 0;
        for (i, row) in matrix.iter_mut().enumerate() {
            if i >= text.len() % cols && index < text.len() {
                row[col] = Some(text.chars().nth(index).unwrap());
                index += 1;
            }
            row_index += 1;
        }
    }

    for i in 0..rows {
        for j in 0..cols {
            if let Some(c) = matrix[i][j] {
                result.push(c);
            }
        }
    }

    result
}
