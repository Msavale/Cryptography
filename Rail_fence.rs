use std::io;

fn main() {
    println!("Rail Fence Cipher Encryption and Decryption");

    // Get user input
    let mut input_text = String::new();
    println!("Enter the text to encrypt:");
    io::stdin().read_line(&mut input_text).expect("Failed to read input.");

    // Remove trailing newline
    input_text = input_text.trim().to_string();

    // Get the key (number of rails) from the user
    println!("Enter the number of rails:");
    let mut key_text = String::new();
    io::stdin().read_line(&mut key_text).expect("Failed to read input.");

    // Parse key to usize
    let key: usize = key_text.trim().parse().expect("Not a valid number");

    // Encrypt the input text
    let encrypted_text = encrypt(&input_text, key);
    println!("Encrypted text: {}", encrypted_text);

    // Decrypt the encrypted text
    let decrypted_text = decrypt(&encrypted_text, key);
    println!("Decrypted text: {}", decrypted_text);
}

// Function to encrypt text using Rail Fence cipher
fn encrypt(text: &str, rails: usize) -> String {
    let mut result = String::new();
    let mut rails_vec: Vec<String> = vec![String::new(); rails];
    let mut rail = 0;
    let mut down = true;

    for c in text.chars() {
        rails_vec[rail].push(c);
        if down {
            rail += 1;
        } else {
            rail -= 1;
        }
        if rail == 0 || rail == rails - 1 {
            down = !down;
        }
    }

    for line in &rails_vec {
        result.push_str(line);
    }

    result
}

// Function to decrypt text using Rail Fence cipher
fn decrypt(text: &str, rails: usize) -> String {
    let mut result = vec![' '; text.len()];
    let mut rail_positions: Vec<usize> = Vec::new();
    let mut rail = 0;
    let mut down = true;

    for (i, c) in text.chars().enumerate() {
        if down {
            rail_positions.push(rail);
            rail += 1;
        } else {
            rail_positions.push(rail);
            rail -= 1;
        }
        if rail == 0 || rail == rails - 1 {
            down = !down;
        }
    }

    for (i, c) in text.chars().enumerate() {
        result[rail_positions[i]] = c;
    }

    result.into_iter().collect()
}
