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

fn matrix_multiply(matrix: &Vec<Vec<i32>>, vector: &[i32; 2]) -> [i32; 2] {
    [
        mod26(matrix[0][0] * vector[0] + matrix[0][1] * vector[1]),
        mod26(matrix[1][0] * vector[0] + matrix[1][1] * vector[1]),
    ]
}

fn modinv(a: i32, m: i32) -> Option<i32> {
    for x in 1..m {
        if (a * x) % m == 1 {
            return Some(x);
        }
    }
    None
}

fn inverse_matrix_2x2(matrix: &Vec<Vec<i32>>) -> Option<Vec<Vec<i32>>> {
    let [[a, b], [c, d]] = [[matrix[0][0], matrix[0][1]], [matrix[1][0], matrix[1][1]]];

    let det = mod26(a * d - b * c);
    let det_inv = modinv(det, 26)?;

    Some(vec![
        vec![mod26(d * det_inv), mod26(-b * det_inv)],
        vec![mod26(-c * det_inv), mod26(a * det_inv)],
    ])
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

pub fn hill_cipher_encrypt(plaintext: String, key: Vec<Vec<i32>>) -> String {
    assert_eq!(key.len(), 2);
    assert!(key.iter().all(|row| row.len() == 2), "Matrix must be 2x2");

    let mut chars: Vec<char> = plaintext
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .collect();

    if chars.len() % 2 != 0 {
        chars.push('X');
    }

    let mut result = String::new();

    for chunk in chars.chunks(2) {
        let vector = [char_to_num(chunk[0]), char_to_num(chunk[1])];
        let product = matrix_multiply(&key, &vector);
        result.push(num_to_char(product[0]));
        result.push(num_to_char(product[1]));
    }

    result
}

pub fn hill_cipher_decrypt(ciphertext: String, key: Vec<Vec<i32>>) -> String {
    let inverse_key = inverse_matrix_2x2(&key).expect("Matrix is not invertible");

    let chars: Vec<char> = ciphertext
        .to_uppercase()
        .chars()
        .filter(|c| c.is_ascii_uppercase())
        .collect();

    assert_eq!(chars.len() % 2, 0, "Ciphertext length is incorrect");

    let mut result = String::new();

    for chunk in chars.chunks(2) {
        let vector = [char_to_num(chunk[0]), char_to_num(chunk[1])];
        let product = matrix_multiply(&inverse_key, &vector);
        result.push(num_to_char(product[0]));
        result.push(num_to_char(product[1]));
    }

    result
}
