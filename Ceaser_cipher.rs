fn main() {
    let text = "HELLO";
    let shift: i32 = 3;
    
    let encrypted = encrypt(text, shift);
    println!("Encrypted: {}", encrypted);
    
    let decrypted = decrypt(&encrypted, shift);
    println!("Decrypted: {}", decrypted);
}

fn encrypt(text: &str, shift: i32) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (((c as i32 - base as i32) + shift).rem_euclid(26) + base as i32) as u8 as char
            } else {
                c
            }
        })
        .collect()
}

fn decrypt(text: &str, shift: i32) -> String {
    encrypt(text, -shift)
}
