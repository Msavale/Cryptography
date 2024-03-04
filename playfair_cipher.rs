use std::io;

fn main() {
    println!("Playfair Cipher Encryption and Decryption");

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
    println!("Enter the key (a keyword without repeated letters):");
    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Failed to read input.");

    // Remove non-alphabetic characters and convert to uppercase
    let key = key.chars()
                 .filter(|&c| c.is_alphabetic())
                 .map(|c| c.to_ascii_uppercase())
                 .collect::<String>();

    // Generate the Playfair matrix
    let matrix = generate_matrix(&key);

    // Encrypt the input text
    let encrypted_text = encrypt(&input_text, &matrix);
    println!("Encrypted text: {}", encrypted_text);

    // Decrypt the encrypted text
    let decrypted_text = decrypt(&encrypted_text, &matrix);
    println!("Decrypted text: {}", decrypted_text);
}

// Function to generate the Playfair matrix
fn generate_matrix(key: &str) -> Vec<Vec<char>> {
    let mut matrix = vec![vec![' '; 5]; 5];
    let mut key_chars = key.chars().collect::<Vec<_>>();
    let mut alphabet = (b'A'..=b'Z')
        .filter(|&c| c != b'J')
        .map(|c| c as char)
        .collect::<Vec<_>>();

    key_chars.dedup(); // Remove repeated letters from the key

    // Fill the matrix with the key
    let mut row = 0;
    let mut col = 0;
    for ch in key_chars {
        matrix[row][col] = ch;
        col += 1;
        if col == 5 {
            col = 0;
            row += 1;
        }
        alphabet.retain(|&x| x != ch); // Remove key character from the alphabet
    }

    // Fill the matrix with the remaining alphabet letters
    for ch in alphabet {
        matrix[row][col] = ch;
        col += 1;
        if col == 5 {
            col = 0;
            row += 1;
        }
    }

    matrix
}

// Function to find the position of a character in the Playfair matrix
fn find_position(matrix: &Vec<Vec<char>>, ch: char) -> (usize, usize) {
    for (i, row) in matrix.iter().enumerate() {
        if let Some(j) = row.iter().position(|&x| x == ch) {
            return (i, j);
        }
    }
    panic!("Character not found in matrix");
}

// Function to encrypt text using Playfair cipher
fn encrypt(text: &str, matrix: &Vec<Vec<char>>) -> String {
    let mut result = String::new();
    let mut chars = text.chars().filter(|&c| c != 'X').collect::<Vec<_>>();
    if chars.len() % 2 != 0 {
        chars.push('X'); // Add 'X' padding if necessary
    }
    for chunk in chars.chunks(2) {
        let (mut row1, mut col1) = find_position(matrix, chunk[0]);
        let (mut row2, mut col2) = find_position(matrix, chunk[1]);
        if row1 == row2 {
            col1 = (col1 + 1) % 5;
            col2 = (col2 + 1) % 5;
        } else if col1 == col2 {
            row1 = (row1 + 1) % 5;
            row2 = (row2 + 1) % 5;
        } else {
            let temp = col1;
            col1 = col2;
            col2 = temp;
        }
        result.push(matrix[row1][col1]);
        result.push(matrix[row2][col2]);
    }
    result
}

// Function to decrypt text using Playfair cipher
fn decrypt(text: &str, matrix: &Vec<Vec<char>>) -> String {
    let mut result = String::new();
    let chars = text.chars().collect::<Vec<_>>();
    for chunk in chars.chunks(2) {
        let (mut row1, mut col1) = find_position(matrix, chunk[0]);
        let (mut row2, mut col2) = find_position(matrix, chunk[1]);
        if row1 == row2 {
            col1 = (col1 + 4) % 5;
            col2 = (col2 + 4) % 5;
        } else if col1 == col2 {
            row1 = (row1 + 4) % 5;
            row2 = (row2 + 4) % 5;
        } else {
            let temp = col1;
            col1 = col2;
            col2 = temp;
        }
        result.push(matrix[row1][col1]);
        result.push(matrix[row2][col2]);
    }
    result
}
