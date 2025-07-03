use nalgebra::{DMatrix, DVector};
use playfair_cipher::{cryptable::Cypher, playfair};

fn char_to_num(c: char) -> i32 {
    c as i32 - 'A' as i32
}

fn num_to_char(n: i32) -> char {
    (mod26(n) as u8 + b'A') as char
}

fn mod26(n: i32) -> i32 {
    ((n % 26) + 26) % 26
}

pub fn caesar_cipher_encrypt(plaintext: String, key: i32) -> String {
    plaintext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let shifted = mod26(char_to_num(c.to_ascii_uppercase()) + key);
                let new_char = num_to_char(shifted);

                if c.is_ascii_uppercase() {
                    new_char
                } else {
                    new_char.to_ascii_lowercase()
                }
            } else {
                c
            }
        })
        .collect()
}

pub fn caesar_cipher_decrypt(ciphertext: String, key: i32) -> String {
    caesar_cipher_encrypt(ciphertext, -key)
}

pub fn vigenere_cipher_encrypt(plaintext: String, key: String) -> String {
    let key_chars: Vec<char> = key.chars().collect();
    let mut key_index = 0;

    plaintext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let is_upper = c.is_ascii_uppercase();

                let plaintext_num = char_to_num(c.to_ascii_uppercase());
                let key_num = char_to_num(key_chars[key_index % key_chars.len()]);
                let encrypted = mod26(plaintext_num + key_num);

                key_index += 1;
                let new_char = num_to_char(encrypted);
                if is_upper {
                    new_char
                } else {
                    new_char.to_ascii_lowercase()
                }
            } else {
                c
            }
        })
        .collect()
}

pub fn vigenere_cipher_decrypt(ciphertext: String, key: String) -> String {
    let key_chars: Vec<char> = key.chars().collect();
    let mut key_index = 0;

    ciphertext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let is_upper = c.is_ascii_uppercase();

                let ciphertext_num = char_to_num(c.to_ascii_uppercase());
                let key_num = char_to_num(key_chars[key_index % key_chars.len()]);
                let decrypted = mod26(ciphertext_num - key_num);

                key_index += 1;
                let new_char = num_to_char(decrypted);
                if is_upper {
                    new_char
                } else {
                    new_char.to_ascii_lowercase()
                }
            } else {
                c
            }
        })
        .collect()
}

pub fn playfair_encrypt(plaintext: String, key: String) -> String {
    let playfair_key = playfair::PlayFairKey::new(key.as_str());
    playfair_key
        .encrypt(plaintext.as_str())
        .unwrap_or_else(|_| String::from("Something went wrong"))
}

pub fn playfair_decrypt(ciphertext: String, key: String) -> String {
    let playfair_key = playfair::PlayFairKey::new(key.as_str());
    playfair_key
        .decrypt(ciphertext.as_str())
        .unwrap_or_else(|_| String::from("Something went wrong"))
}

pub fn hill_cipher_encrypt(plaintext: String, key: DMatrix<i32>) -> Option<String> {
    let size = key.ncols();

    if key.nrows() != size {
        return None;
    }

    let mut filtered_chars = vec![];
    for c in plaintext.chars() {
        if c.is_ascii_alphabetic() {
            filtered_chars.push((c.to_ascii_uppercase(), c.is_ascii_uppercase()));
        }
    }

    let padding_needed = (size - filtered_chars.len() % size) % size;
    for _ in 0..padding_needed {
        filtered_chars.push(('X', true));
    }

    let mut ciphertext = String::new();

    for chunk in filtered_chars.chunks(size) {
        let input_vec: Vec<i32> = chunk.iter().map(|(c, _)| char_to_num(*c)).collect();
        let vec = DVector::from_column_slice(&input_vec);
        let result = key.clone() * vec;

        for (i, val) in result.iter().enumerate() {
            let ch = num_to_char(mod26(*val));
            let is_upper = chunk[i].1;
            if is_upper {
                ciphertext.push(ch);
            } else {
                ciphertext.push(ch.to_ascii_lowercase());
            }
        }
    }

    Some(ciphertext)
}
